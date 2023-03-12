#[macro_use]
extern crate log;
extern crate diesel;
//See https://github.com/actix/examples/tree/master/databases/diesel
use actix_web::{
    middleware::Logger,
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use reqwest::Client;
use std::env;

mod models;
mod schema;
mod sql_actions;

async fn index(path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    let frames: Vec<models::Frame> = sql_actions::get_frames(user_id);
    if frames.len() == 0 {
        return HttpResponse::BadRequest()
            .body("The user ID supplied was not found, or the user has no data to send.");
    }
    //takes the mean of sensor data
    let avg_values: models::DataOut = {
        let n_frames = frames.len() as f32;
        let mut boiler_bool_count = 0.0;
        let mut avg_temp = 0.0;
        let mut avg_ppm = 0.0;
        let mut avg_light = 0.0;
        for i in frames {
            avg_ppm += i.ppm;
            avg_temp += i.temp;
            avg_light += i.light;
            if i.boiler {
                boiler_bool_count += 1.0;
            }
        }
        //If the boiler is on for more than half of the frames, then set the average to be on
        let avg_boiler = {
            if boiler_bool_count >= n_frames / 2.0 {
                true
            } else {
                false
            }
        };
        avg_temp /= n_frames;
        avg_ppm /= n_frames;
        avg_light /= n_frames;
        models::DataOut {
            temp: avg_temp,
            ppm: avg_ppm,
            light: avg_light,
            boiler: avg_boiler,
        }
    };
    HttpResponse::Ok().json(Json(avg_values))
}

async fn push(frame: web::Json<models::FrameIn>) -> impl Responder {
    let uuids = sql_actions::get_uuids();
    if !uuids.contains(&frame.0.uid) {
        return HttpResponse::BadRequest().body(
            "UUID in request was not found in users. Create a user first and use the UUID supplied.",
        );
    };
    let utc: DateTime<Utc> = Utc::now();
    let utcstr = utc.to_string();
    let final_frame = models::Frame {
        uid: frame.uid.clone(),
        datetime: utcstr,
        temp: frame.temp,
        ppm: frame.ppm,
        light: frame.light,
        boiler: frame.boiler,
    };
    sql_actions::add_frame(final_frame);
    HttpResponse::Ok().body("Successfully appended frame.")
}

async fn toggle_appliance(
    user_ip: String,
    toggle: web::Json<models::ApplianceToggle>,
) -> impl Responder {
    let client = Client::new();
    let content = toggle.0;
    let body: Json<models::ApplianceToggle> = Json(content);
    //sends a POST to a small listener on the user's raspberry pi. This will cause the respective
    //Appliance to toggle, through communication to the ESP32 microcontroller
    let res = client.post(user_ip).form(&body).send().await;
    HttpResponse::new(res.unwrap().status())
}

async fn create_user(form: web::Json<models::UserIn>) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_bytes: &[u8] = form.0.password.as_bytes();
    let password_hash = argon2
        .hash_password(password_bytes, &salt)
        .expect("Failed to hash password")
        .to_string();
    let baseuser = models::BaseUser {
        fname: form.0.fname,
        lname: form.0.lname,
        address: form.0.address,
    };
    let uservec = sql_actions::get_users();
    if uservec.contains(&baseuser) {
        return HttpResponse::BadRequest().body(format!(
            "This user already exists with uid {}",
            sql_actions::get_user(baseuser).id
        ));
    }
    let uuid = uuid::Uuid::new_v4().to_string();
    let user = models::User {
        id: uuid.clone(),
        psk_hash: password_hash,
        address: baseuser.address,
        fname: baseuser.fname,
        lname: baseuser.lname,
    };
    sql_actions::insert_user(user);
    HttpResponse::Ok().body(format!("Successfully created user. UUID is {uuid}"))
}

async fn delete_user(user: web::Json<models::UserIn>) -> impl Responder {
    let baseuser = models::BaseUser {
        fname: user.fname.clone(),
        lname: user.lname.clone(),
        address: user.address.clone(),
    };
    let uservec = sql_actions::get_users();
    if !uservec.contains(&baseuser) {
        return HttpResponse::NotFound().body("User not found.");
    }
    let user_full = sql_actions::get_user(baseuser);
    let hash = PasswordHash::new(&user_full.psk_hash).expect("Failed to hash password");
    let pass = user.password.as_bytes();
    match Argon2::default().verify_password(pass, &hash) {
        Ok(()) => {
            sql_actions::delete_user(user_full.id.clone());
            HttpResponse::Ok().body(format!(
                "Successfully deleted user with id {}",
                user_full.id
            ))
        }
        Err(e) => HttpResponse::Forbidden().body(format!("Permission denied: {e}")),
    }
}

async fn get_uuid(form: web::Json<models::UserIn>) -> impl Responder {
    let baseuser = models::BaseUser {
        fname: form.fname.clone(),
        lname: form.lname.clone(),
        address: form.address.clone(),
    };
    let user = sql_actions::get_user(baseuser);
    let hash = PasswordHash::new(&user.psk_hash).expect("Failed to parse password hash");
    let pass = form.password.as_bytes();
    match Argon2::default().verify_password(pass, &hash) {
        Ok(()) => return HttpResponse::Ok().body(user.id),
        Err(e) => {
            return HttpResponse::Unauthorized().body(format!("Incorrect password: {e}"));
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::process::exit;
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    let host = env::var("HOST").unwrap_or_else(|e| {
        println!("{e}");
        exit(1)
    });
    let port = env::var("PORT").unwrap_or_else(|e| {
        println!("{e}");
        exit(1)
    });
    env_logger::init();
    info!("Starting server on {}:{}", host, port);
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .route("/{user_id}", web::get().to(index))
            .route("/get_uid", web::post().to(get_uuid))
            .route("/append", web::post().to(push))
            .route("/toggle", web::post().to(toggle_appliance))
            .route("/create_user", web::post().to(create_user))
            .route("/delete_user", web::post().to(delete_user))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

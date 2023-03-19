//See https://github.com/actix/examples/tree/master/databases/diesel
use actix_cors::Cors;
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
use log::{error, info, warn};
use log4rs;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use reqwest::Client;
use std::env;

mod models;
mod schema;
mod sql_actions;

async fn get_average_data(path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    let frames: Vec<models::Frame> = sql_actions::get_frames(user_id.clone());
    if frames.len() == 0 {
        warn!("Bad request /data/{}/average", user_id);
        return HttpResponse::BadRequest()
            .body("The user ID supplied was not found, or the user has no data to send.");
    }
    //takes the mean of sensor data
    let avg_values: models::DataOut = {
        let n_frames = frames.len() as f32;
        //let mut boiler_bool_count = 0;
        let mut avg_temp = 0.0;
        let mut avg_ppm = 0.0;
        let mut avg_humidity = 0.0;
        for i in frames {
            avg_ppm += i.ppm;
            avg_temp += i.temp;
            avg_humidity += i.humidity;
            //if i.boiler{
            //    boiler_bool_count += 1;
            //}
        }
        //If the boiler is on for more than half of the frames, then set the average to be on
        //let avg_boiler = {
        //    if boiler_bool_count >= n_frames / 2.0 {
        //        true
        //    } else {
        //        false
        //    }
        //};
        //Take mean of all numeric data
        avg_temp /= n_frames;
        avg_ppm /= n_frames;
        avg_humidity /= n_frames;
        models::DataOut {
            temp: avg_temp,
            ppm: avg_ppm,
            humidity: avg_humidity,
        }
    };
    HttpResponse::Ok().json(Json(avg_values))
}

//Get list of all sensor datas
async fn get_list_data(path: web::Path<String>) -> impl Responder {
    let uid = path.into_inner();
    let datas = sql_actions::get_frames(uid.clone());
    if datas.len() == 0 {
        warn!("Bad request /data/{uid}/list");
        return HttpResponse::BadRequest()
            .body("The user ID supplied was not found, or the user has no data to send.");
    }
    let sensor_datas = {
        let mut datetimes: Vec<String> = vec![];
        let mut temps: Vec<f32> = vec![];
        let mut ppms: Vec<f32> = vec![];
        let mut humidities: Vec<f32> = vec![];
        //let mut boilers: Vec<bool> = vec![];
        for i in datas {
            datetimes.push(i.datetime);
            temps.push(i.temp);
            ppms.push(i.ppm);
            humidities.push(i.humidity);
            //boilers.push(i.boiler);
        }
        models::SensorDatas {
            datetimes,
            temps,
            ppms,
            humidities,
            //boilers,
        }
    };
    return HttpResponse::Ok().json(Json(sensor_datas));
}

//Push new sensor data
async fn push(frame: web::Json<models::FrameIn>) -> impl Responder {
    let uuids = sql_actions::get_uuids();
    if !uuids.contains(&frame.0.uid) {
        warn!("Bad request: POST /data/{}", frame.0.uid);
        return HttpResponse::BadRequest().body(
            "UUID in request was not found in existing users. Create a user first and use the UUID supplied.",
        );
    };
    //Define datetime serverside
    let utc: DateTime<Utc> = Utc::now();
    let utcstr = utc.to_string();
    let final_frame = models::Frame {
        uid: frame.uid.clone(),
        datetime: utcstr,
        temp: frame.temp,
        ppm: frame.ppm,
        humidity: frame.humidity,
        //boiler: frame.boiler,
    };
    sql_actions::add_frame(final_frame);
    HttpResponse::Ok().body("Successfully appended frame.")
}

//Toggle user's lights or boiler
async fn toggle_appliance(
    user_ip: String,
    toggle: web::Json<models::ApplianceToggle>,
) -> impl Responder {
    let client = Client::new();
    let content = toggle.0;
    let body: Json<models::ApplianceToggle> = Json(content);
    //sends a PUT to a small listener on the user's raspberry pi. This will cause the respective
    //Appliance to toggle, through communication to the ESP32 microcontroller
    let res = client.put(user_ip).form(&body).send().await;
    HttpResponse::new(res.unwrap().status())
}

//Creates new user, hashing password
async fn create_user(form: web::Json<models::UserIn>) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_bytes: &[u8] = form.0.password.as_bytes();
    //Argon2 password hashes include salt
    let password_hash = argon2
        .hash_password(password_bytes, &salt)
        .unwrap_or_else(|e| {
            error!("Failed to hash password: {e}");
            std::process::exit(1)
        })
        .to_string();
    let baseuser = models::BaseUser {
        fname: form.0.fname,
        lname: form.0.lname,
        address: form.0.address,
    };
    let uservec = sql_actions::get_users();
    //Return error if user already exists
    if uservec.contains(&baseuser) {
        return HttpResponse::Conflict().body(sql_actions::get_user_from_baseuser(baseuser).id);
    }
    //Do the actual user creation
    let uuid = uuid::Uuid::new_v4().to_string();
    let user = models::User {
        id: uuid.clone(),
        psk_hash: password_hash,
        address: baseuser.address,
        fname: baseuser.fname,
        lname: baseuser.lname,
    };
    sql_actions::insert_user(user);
    HttpResponse::Created().body(format!("{uuid}"))
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
    let user_full = sql_actions::get_user_from_baseuser(baseuser);
    //Verify password hash
    let hash = PasswordHash::new(&user_full.psk_hash).unwrap_or_else(|e| {
        error!("Failed to parse pskhash: {e}");
        std::process::exit(1);
    });
    let pass = user.password.as_bytes();
    match Argon2::default().verify_password(pass, &hash) {
        Ok(()) => {
            sql_actions::delete_user(user_full.id.clone());
            HttpResponse::Ok().body(format!(
                "Successfully deleted user with id {}",
                user_full.id
            ))
        }
        //Incorrect password returns error
        Err(e) => {
            warn!("Incorrect password to {}", user_full.id);
            HttpResponse::Forbidden().body(format!("Permission denied: {e}"))
        }
    }
}

async fn get_uuid(user_in: web::Query<models::UserIn>) -> impl Responder {
    let baseuser = models::BaseUser {
        fname: user_in.fname.clone(),
        lname: user_in.lname.clone(),
        address: user_in.address.clone(),
    };
    let user = sql_actions::get_user_from_baseuser(baseuser);
    let hash = PasswordHash::new(&user.psk_hash).expect("Failed to parse password hash");
    let pass = user_in.password.as_bytes();
    match Argon2::default().verify_password(pass, &hash) {
        Ok(()) => return HttpResponse::Ok().body(user.id),
        Err(e) => {
            return {
                warn!("Incorrect password to {}", user.id);
                HttpResponse::Unauthorized().body(format!("Incorrect password: {e}"))
            };
        }
    }
}

async fn get_user_from_uuid(path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    let user = sql_actions::get_user_from_id(user_id).unwrap_or_else(|e| {
        error!("{e}");
        std::process::exit(1)
    });
    return HttpResponse::Ok().json(Json(user));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    use std::process::exit;
    dotenv().ok();
    //Verbose error messages
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
    //Setup openSSL, currently uses certificates in server directory
    let privkey = env::var("PRIVKEYFILE").unwrap_or_else(|e| {
        println!("{e}");
        exit(1)
    });
    let cert = env::var("CERTFILE").unwrap_or_else(|e| {
        println!("{e}");
        exit(1)
    });
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(privkey, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(cert).unwrap();
    info!("Starting server on {}:{}", host, port);
    //Spawn a new thread for the server and its endpoints
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(["GET", "POST", "PUT", "DELETE"])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            //Get user ID from data
            .route("/user/", web::get().to(get_uuid))
            //Get user data from ID
            .route("/user/{user_id}", web::get().to(get_user_from_uuid))
            //Create new user, providing it doesn't exist
            .route("/user/", web::post().to(create_user))
            //Delete user, Providing it exists.
            .route("/user/", web::delete().to(delete_user))
            //Toggle user's boiler/lights
            .route("/user/appliance", web::put().to(toggle_appliance))
            //Get average of all sensor data from specific user
            .route("/data/{user_id}/average", web::get().to(get_average_data))
            //Get list of all sensor data from specific user
            .route("/data/{user_id}/list", web::get().to(get_list_data))
            //Append new sensor data to frames table
            .route("/data/", web::post().to(push))
            .service(web::redirect(
                "/",
                "https://github.com/TheGraphVertices/Backend/blob/main/schema.md",
            )) //Requests to root redirect to docs
    })
    //Bind HTTPS
    .bind_openssl(format!("{}:{}", host, port), builder)?
    .run()
    .await
}

#[macro_use]
extern crate log;
extern crate diesel;

//See https://github.com/actix/examples/tree/master/databases/diesel
use actix_web::{
    middleware::Logger,
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};
use dotenvy::dotenv;
use reqwest::Client;
use std::env;

mod models;
mod schema;
mod sql_actions;

async fn index(path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    let frames: Vec<models::Frame> = sql_actions::get_frames(user_id);
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
    Json(avg_values)
}

async fn push(frame: web::Json<models::Frame>) -> impl Responder {
    sql_actions::add_frame(frame.0);
    HttpResponse::Ok().finish()
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
    HttpResponse::Ok().status(res.unwrap().status()).finish()
}

async fn create_user(form: web::Json<models::UserIn>) -> impl Responder {
    let userin = form.0;
    let uuid = uuid::Uuid::new_v4().to_string();
    let user = models::User {
        id: uuid.clone(),
        address: userin.address,
        fname: userin.fname,
        lname: userin.lname,
    };
    sql_actions::insert_user(user);
    uuid
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
            .route("/append", web::post().to(push))
            .route("/toggle", web::post().to(toggle_appliance))
            .route("/create_user", web::post().to(create_user))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

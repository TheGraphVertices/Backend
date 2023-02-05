#[macro_use]
extern crate log;

use std::env;
use actix_web::{get, App, HttpServer, Responder, middleware::Logger};
use dotenvy::dotenv;


#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    let host = env::var("HOST").expect("Environment variable HOST not set.");
    let port = env::var("PORT").expect("Environment variable PORT not set.");
    env_logger::init();
    info!("Starting server on {}.{}",host,port);
    HttpServer::new(move ||{
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(index)
        })
        .bind(format!("{}:{}",host,port))?
        .run()
        .await
}
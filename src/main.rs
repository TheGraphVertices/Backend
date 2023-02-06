#[macro_use]
extern crate log;

use actix_web::{get, middleware::Logger, App, HttpServer, Responder};
use dotenvy::dotenv;
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    let host = match env::var("HOST") {
        Ok(val) => val,
        Err(e) => {
            println!("Finding HOST env var failed: {e}");
            std::process::exit(1)
        }
    };
    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(e) => {
            println!("Finding PORT env var failed: {e}");
            std::process::exit(1)
        }
    };
    env_logger::init();
    info!("Starting server on {}.{}", host, port);
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(index)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

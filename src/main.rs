#[macro_use]
extern crate log;
extern crate diesel;

use actix_web::{get, middleware::Logger, post, App, Error, HttpServer, Responder};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use dotenvy::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;
use uuid::Uuid;

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
    //For demonstration purposes, I've created a cert in the repo with password 'vertex'.
    //DO NOT
    //USE IN PRODUCTION.
    let mut ssl_builder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls()).expect("Failed to create ssl builder");
    ssl_builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .expect(
            "Didn't find SSL private key: key.pem, create one with the command:
            openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -sha256 -subj '/C=Country/ST=State/L=Locality/O=Name/OU=Org/CN=Name'
            ",
        );
    ssl_builder
        .set_certificate_chain_file("cert.pem")
        .expect("Didn't find cert.pem.");
    env_logger::init();
    info!("Starting server on {}:{}", host, port);
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(index)
    })
    .bind_openssl(format!("{}:{}", host, port), ssl_builder)?
    .run()
    .await
}

#[macro_use]
extern crate log;
extern crate diesel;

//See https://github.com/actix/examples/tree/master/databases/diesel
use actix_web::{
    get, middleware::Logger, post, web, App, Error, HttpResponse, HttpServer, Responder,
};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use dotenvy::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

mod models;
mod schema;
mod sql_actions;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

/*#[post("/append")]
async fn push<'a>(frame: web::Json<models::Frame<'a>>) -> impl Responder {
    sql_actions::add_frame(frame.0);
    HttpResponse::Ok().finish()
}*/

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
    info!("Password for key is 'vertex'");
    ssl_builder
        .set_certificate_chain_file("cert.pem")
        .expect("Didn't find cert.pem.");
    env_logger::init();
    info!("Starting server on {}:{}", host, port);
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(index).service(push)
    })
    .bind_openssl(format!("{}:{}", host, port), ssl_builder)?
    .run()
    .await
}

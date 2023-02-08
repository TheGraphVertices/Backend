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

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[post("/append")]
//Function to append to SQL lists
async fn append_to_lists(
    pool: web::Data<DbPool>,
    form: web::Json<models::DataIn>,
) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let data = models::DataIn {
        temp: form.temp,
        ppm: form.ppm,
        light: form.light,
        boiler_on: form.boiler_on,
        uid: form.uid,
    };
    web::block(move || {
        let mut conn = pool.get().expect("Failed to create SQL connection pool.");
        diesel::insert_into(users).values(data).execute(&mut conn);
    });
    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    let sqlfile = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(e) => {
            println!("{e}");
            std::process::exit(1);
        }
    };
    let sql_manager = ConnectionManager::<SqliteConnection>::new(sqlfile);
    let sql_pool = r2d2::Pool::builder()
        .build(sql_manager)
        .expect("Failed to create SQL pool.");
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
    info!("Password for key is 'Vertex'");
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(sql_pool.clone()))
            .service(index)
            .service(append_to_lists)
    })
    .bind_openssl(format!("{}:{}", host, port), ssl_builder)?
    .run()
    .await
}

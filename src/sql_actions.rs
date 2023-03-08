use crate::models::{Frame, User};
use crate::schema;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use std::{env::var, process::exit};

fn establish_sql_connection() -> SqliteConnection {
    let sqlfile = var("DATABASE_URL").expect("Didn't find DATABASE_URL env var.");
    SqliteConnection::establish(&sqlfile).expect("Failed to establish SQL connection.")
}

pub fn add_frame(frame: Frame) {
    let mut conn = establish_sql_connection();
    diesel::insert_into(schema::frame::table)
        .values(&frame)
        .execute(&mut conn)
        .unwrap_or_else(|e| {
            println!("{e}");
            exit(1)
        });
}

pub fn insert_user(user: User) {
    let mut conn = establish_sql_connection();
    diesel::insert_into(schema::users::table)
        .values(&user)
        .execute(&mut conn)
        .unwrap_or_else(|e| {
            println!("{e}");
            exit(1)
        });
}
pub fn get_frames(uid_in: String) -> Vec<Frame> {
    use schema::frame::dsl::*;
    let mut conn = establish_sql_connection();
    frame.filter(uid.is(uid_in)).load(&mut conn).unwrap()
}

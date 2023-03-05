use crate::models::{Frame, User};
use crate::schema;
use diesel::prelude::*;
use std::{env::var, process::exit};

fn establish_sql_connection() -> SqliteConnection {
    let sqlfile = var("DATABASE_URL").unwrap_or_else(|e| {
        println!("{e}");
        exit(1)
    });
    SqliteConnection::establish(&sqlfile).unwrap_or_else(|e| {
        println!("{e}");
        exit(1)
    })
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

pub fn create_user(user: User) {
    let mut conn = establish_sql_connection();
    diesel::insert_into(schema::users::table)
        .values(&user)
        .execute(&mut conn)
        .unwrap_or_else(|e| {
            println!("{e}");
            exit(1)
        });
}
/*
pub fn get_frame<'a>(datetime_in: &'a str, uid_in: &'a str) -> Frame<'a> {
    use crate::schema::frame::dsl::*;
    let conn = establish_sql_connection();
    let data = schema::frame::table
        .filter(datetime.eq(datetime_in))
        .filter(uid.eq(uid_in))
        .first(&mut conn)
        .expect("Failed to load frame with uid {uid_in} and datetime {datetime_in}");
    data
}*/

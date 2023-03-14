use crate::models;
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
    diesel::insert_into(schema::frames::table)
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

pub fn delete_user(uid_in: String) {
    use schema::frames::dsl::*;
    use schema::users::dsl::*;
    let mut conn = establish_sql_connection();
    //Preserve referential integrity by deleting both frames matching user's ID and user itself
    diesel::delete(frames)
        .filter(uid.is(uid_in.clone()))
        .execute(&mut conn)
        .unwrap();
    diesel::delete(users)
        .filter(id.is(uid_in))
        .execute(&mut conn)
        .unwrap();
}

pub fn get_uuids() -> Vec<String> {
    use schema::users::dsl::*;
    let mut conn = establish_sql_connection();
    users.select(id).load(&mut conn).unwrap()
}

pub fn get_user_from_baseuser(u: models::BaseUser) -> models::User {
    use schema::users::dsl::*;
    let mut conn = establish_sql_connection();
    users
        .filter(fname.is(u.fname))
        .filter(lname.is(u.lname))
        .filter(address.is(u.address))
        .first(&mut conn)
        .unwrap()
}

pub fn get_user_from_id(id_in: String) -> models::User {
    use schema::users::dsl::*;
    let mut conn = establish_sql_connection();
    users.filter(id.is(id_in)).first(&mut conn).unwrap()
}

pub fn get_users() -> Vec<models::BaseUser> {
    use schema::users::dsl::*;
    let mut conn = establish_sql_connection();
    let uservec: Vec<models::User> = users.load(&mut conn).unwrap();
    let mut baseusers: Vec<models::BaseUser> = vec![];
    for i in uservec {
        baseusers.push(models::BaseUser {
            fname: i.fname,
            lname: i.lname,
            address: i.address,
        })
    }
    baseusers
}

pub fn get_frames(uid_in: String) -> Vec<Frame> {
    use schema::frames::dsl::*;
    let mut conn = establish_sql_connection();
    frames.filter(uid.is(uid_in)).load(&mut conn).unwrap()
}

use crate::schema::*;
//use diesel::sql_types::{Bool, Float, Text};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataOut {
    pub temp: f32,
    pub ppm: f32,
    pub light: f32,
    pub boiler: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIn {
    pub fname: String,
    pub lname: String,
    pub address: String,
    pub password: String,
}

#[derive(PartialEq)]
pub struct BaseUser {
    pub fname: String,
    pub lname: String,
    pub address: String,
}

#[derive(Deserialize, Serialize)]
pub struct FrameIn {
    //Compound primary key of uid (user id) and datetime
    pub uid: String,
    pub temp: f32,
    pub ppm: f32,
    pub light: f32,
    pub boiler: bool,
}

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
pub struct Frame {
    //Compound primary key of uid (user id) and datetime
    pub uid: String,
    pub datetime: String,
    pub temp: f32,
    pub ppm: f32,
    pub light: f32,
    pub boiler: bool,
}

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
pub struct User {
    pub id: String,       //Primary key of id
    pub psk_hash: String, //Argon2 password hash also contains salt
    pub fname: String,
    pub lname: String,
    pub address: String,
}

#[derive(Serialize, Deserialize)]
enum Appliances {
    Boiler,
    Lights,
}

#[derive(Serialize, Deserialize)]
pub struct ApplianceToggle {
    uid: String,
    appliance_type: Appliances,
    on_off: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorDatas {
    pub datetimes: Vec<String>,
    pub temps: Vec<f32>,
    pub ppms: Vec<f32>,
    pub lights: Vec<f32>,
    pub boilers: Vec<bool>,
}

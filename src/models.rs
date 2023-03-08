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
#[derive(Deserialize)]
pub struct UserIn {
    pub fname: String,
    pub lname: String,
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = frame)]
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
#[diesel(table_name = users)]
pub struct User {
    pub id: String, //Primary key of id
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

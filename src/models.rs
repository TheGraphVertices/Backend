use crate::schema::data_ins;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
pub struct DataIn {
    pub uid: i32,
    pub temp: i32,
    pub ppm: i32,
    pub light: i32,
    pub boiler_on: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataOut {
    pub temps: Vec<i32>,
    pub ppms: Vec<u32>,
    pub lights: Vec<u16>,
    pub avg_temp: f32,
    pub avg_ppm: u32,
    pub avg_light: u16,
}

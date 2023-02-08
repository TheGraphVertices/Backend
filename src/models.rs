use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct DataIn {
    pub temp: i32,
    pub ppm: u32,
    pub light: u16,
    pub boiler_on: bool,
    pub uid: String,
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

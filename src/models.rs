use crate::schema::*;
//use diesel::sql_types::{Bool, Float, Text};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataIn<'a> {
    temp: i32,
    ppm: i32,
    light: i32,
    boiler_on_off: bool,
    uid: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataOut {
    avg_temp: i32,
    avg_ppm: i32,
    avg_light: i32,
    temps: Vec<i32>,
    lights: Vec<i32>,
    boiler_bools: Vec<bool>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "frame"]
pub struct Frame<'a> {
    //Compound primary key of uid (user id) and datetime
    uid: &'a str,
    datetime: &'a str,
    temp: f32,
    ppm: f32,
    light: f32,
    boiler: bool,
}

/*
impl<'a, B: diesel::backend::Backend> Queryable<(Text, Text, Float, Float, Float, Bool), B>
    for Frame<'a>
{
    type Row = (&'a str, &'a str, f32, f32, f32, bool);
    fn build(row: Self::Row) -> deserialize::Result<Self> {
        let fr = Frame {
            uid: row.0,
            datetime: row.1,
            temp: row.2,
            ppm: row.3,
            light: row.4,
            boiler: row.5,
        };
        Ok(fr)
    }
}*/

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
pub struct User<'a> {
    id: &'a str, //Primary key of id
    fname: &'a str,
    lname: &'a str,
    address: &'a str,
}

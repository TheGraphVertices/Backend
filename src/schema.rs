use diesel::prelude::*;
table! {
    data_ins (uid){
        uid -> Integer,
        temp -> Integer,
        ppm -> Integer,
        light -> Integer,
        boiler_on -> Bool,
    }
}

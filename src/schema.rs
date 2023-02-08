use diesel::prelude::*;
table! {
    users (uid) {
        uid -> Integer,
    }
}
table! {
    temps (uid, temp) {
        uid -> Integer,
        temp -> Integer,
    }
}
table! {
    ppms (uid, ppm) {
        uid -> Integer,
        ppm -> Integer,
    }
}
table! {
    lights (uid, light) {
        uid -> Integer,
        light -> Integer,
    }
}

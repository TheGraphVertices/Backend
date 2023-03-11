// @generated automatically by Diesel CLI.

diesel::table! {
    frames (uid, datetime) {
        uid -> Text,
        datetime -> Text,
        temp -> Float,
        ppm -> Float,
        light -> Float,
        boiler -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        psk_hash -> Text,
        salt -> Text,
        fname -> Text,
        lname -> Text,
        address -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(frames, users,);

// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 64]
        password -> Varchar,
        #[max_length = 16]
        student_id -> Varchar,
        entered_room_id -> Nullable<Int8>,
    }
}

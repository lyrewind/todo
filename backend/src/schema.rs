// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        title -> Text,
        details -> Text,
        status_code -> Int4,
    }
}

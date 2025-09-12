// @generated automatically by Diesel CLI.

diesel::table! {
    ip_logs (id) {
        id -> Int4,
        original_ip -> Text,
        reversed_ip -> Text,
        created_at -> Timestamptz,
    }
}


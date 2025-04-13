// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (uuid) {
        uuid -> Text,
        amount -> Integer,
        client_modified_at -> Timestamp,
        exponent -> Integer,
        merchant -> Text,
        occurred_at -> Timestamp,
        parent -> Text,
        server_modified_at -> Timestamp,
    }
}

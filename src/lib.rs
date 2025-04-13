use chrono::NaiveDateTime;
use diesel::prelude::*;

mod schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct Transaction {
    uuid: String,
    amount: i32,
    client_modified_at: NaiveDateTime,
    exponent: i32,
    merchant: String,
    occurred_at: NaiveDateTime,
    parent: String,
    server_modified_at: NaiveDateTime,
}

async fn open_db() {}

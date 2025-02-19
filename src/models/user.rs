use chrono::{DateTime, Utc};
use diesel::{prelude::*, sql_query};
use serde::{Deserialize, Serialize};

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        created_at -> Timestamptz,
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(AsChangeset, Deserialize, Debug)]
#[table_name = "users"]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}

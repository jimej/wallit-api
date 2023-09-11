use crate::schema::users;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;
#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    id: Uuid,
    username: String,
    email: String,
    first_name: String,
    last_name: String,
    created_at: NaiveDateTime,
    last_modified: NaiveDateTime,
}

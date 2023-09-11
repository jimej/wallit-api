use crate::schema::users;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
// use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
     email: String,
     first_name: String,
     last_name: String,
    pub created_at: Option<NaiveDateTime>,
    pub last_modified: NaiveDateTime,
}

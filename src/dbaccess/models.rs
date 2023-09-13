use crate::schema::{logins, groups, users};
use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable, Associations, Identifiable};
// use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
// #[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
#[diesel(table_name = logins)]
pub struct Login {
    pub id: Uuid,
    pub group_id: Uuid,   // this should be Option<Uuid> indicating default_group
    cname: String,
    url: Option<String>,
    login: String,
    password: String,
    email: String,
    description: String,

    pub created_at: Option<NaiveDateTime>,
    pub last_modified: NaiveDateTime,
}

#[derive(Queryable, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = groups)]
pub struct Group {
  id: Uuid,
  group_name: String,
  user_id: Uuid,
  team: bool,
  pub created_at: Option<NaiveDateTime>,
  pub last_modified: NaiveDateTime,
}

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
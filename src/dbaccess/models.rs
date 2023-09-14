use crate::schema::{logins, groups, users};
use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable, Associations, Identifiable};
// use diesel::prelude::*;
use uuid::Uuid;
use serde::Deserialize;

#[derive(Queryable, Insertable, Deserialize)] // NaiveDateTime needs chrono serde feature to serialize
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
#[diesel(table_name = logins)]
pub struct Login {
    // pub id: Uuid,
    pub group_id: Option<Uuid>,   // this should be Option<Uuid> indicating default_group
    pub cname: String,
    pub url: Option<String>,
    pub login: String,
    pub password: String,
    pub email: String,
    pub description: Option<String>,
    // pub created_at: Option<NaiveDateTime>,
    pub last_modified: Option<NaiveDateTime>,
    pub user_id: Option<Uuid>,
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
  pub last_modified: Option<NaiveDateTime>,
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
    pub last_modified: Option<NaiveDateTime>,
}
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use super::models::Login;
use crate::schema::logins::dsl::logins;
pub fn add_login(l: &Login, conn: &mut PooledConnection<ConnectionManager<PgConnection>>) { // default_group
    // validate user_id or group_id is not null
    // or both exist, verify group_id belongs to the user_id
    let res = diesel::insert_into(logins).values(l).execute(conn);
    match res {
        Err(e) => println!("{e}"),
        Ok(_) => println!("great!!!! new login added successfully"),
    }
}

pub fn get_login() { // default_group

}

pub fn update_login() {

}

pub fn duplicate_login() { // how about prevent sharing

}

pub fn move_login() { 
    
}

pub fn delete_login() {

}

pub fn add_group() { // team: true, false

}

pub fn get_group() { // default_group

}

pub fn update_group() { // can not change team true -> fales or fales -> true

}

pub fn delete_group() { // may require the group is sempty
    
}
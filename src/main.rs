use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use wallit_api::get_connection_pool;
// use wallit_api::*;
// use models::*;
mod models;
mod schema;

use chrono::NaiveDateTime;

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let pool = get_connection_pool();

    let app = Router::new()
        // .with_state(pool)
        .route("/", get(|| async { "hello world" }))
        .route("/user", get(get_user))
        .route("/add", get(add_user))
        .with_state(pool); // with_state at last step

    axum::Server::bind(&addr) // or bind(&"127.0.0.1:8030".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}

type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

async fn add_user(State(pool): State<ConnectionPool>) {
    let mut conn = pool.get().unwrap();
    use schema::users::dsl::*;

    let now = SystemTime::now();
    let millis = now.duration_since(UNIX_EPOCH).unwrap().as_millis();
    let last = NaiveDateTime::from_timestamp_millis(millis as i64);
    let records = [(
        username.eq("user3"),
        email.eq("user3@example.gmail.com"),
        first_name.eq("First"),
        last_name.eq("Last"),
        last_modified.eq(last.unwrap()),
    )];
    let _res = diesel::insert_into(users)
        .values(&records)
        .execute(&mut conn);
    match _res {
        Err(e) => println!("{e}"),
        Ok(_) => println!("great!!!!"),
    }

}

async fn get_user(State(pool): State<ConnectionPool>) -> &'static str {
    let mut conn = pool.get().unwrap();
    use schema::users::dsl::*;
    let results = users
        // .filter(published.eq(true))
        .limit(5)
        .load::<models::User>(&mut conn)
        .expect("Error loading posts");
    for u in results {
        println!(
            "{} {} {:?} {:?}  ",
            u.username, u.id, u.created_at, u.last_modified
        );
    }
    ""
}

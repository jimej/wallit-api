use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use wallit_api::get_connection_pool;

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    let app = Router::new()
        .layer(Extension(get_connection_pool()))
        .route("/", get(|| async { "hello world" }));

    axum::Server::bind(&addr) // or bind(&"127.0.0.1:8030".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}

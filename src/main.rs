mod env;

use axum::{
    routing::get,
    Router,
};
use log::{info};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let port = env::required_u16("PORT");
    let addr = format!("0.0.0.0:{}", port);

    info!("start server at {}", addr);

    // build our application with a single route
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/callback", get(callback));

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn callback() -> String {
    info!("callback");
    "OK".to_string()
}
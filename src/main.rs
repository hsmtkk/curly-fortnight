mod env;

use axum::{
    routing::get,
    Router,
};
use tracing::{event, span, Level};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let span = span!(Level::INFO, "main");
    let _guard = span.enter();

    let port = env::required_u16("PORT");
    let addr = format!("0.0.0.0:{}", port);
    event!(Level::INFO, "start server at {}", addr);

    // build our application with a single route
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/callback", get(callback))
    .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn callback(body: String) -> String {
    let span = span!(Level::INFO, "callback");
    let _guard = span.enter();
    event!(Level::INFO, "{}", body);
    "OK".to_string()
}
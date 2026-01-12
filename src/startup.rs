use crate::routes::*;
use axum::{
    Router,
    routing::{get, post},
};

pub async fn run(address: String) {
    let app = app();

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let _ = axum::serve(listener, app).await;
}

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

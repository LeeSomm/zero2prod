use crate::routes::{health_check, subscribe};
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};
use http::Request;
use tokio::net::TcpListener;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::TraceLayer,
};

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .and_then(|id| id.header_value().to_str().ok())
                    .unwrap_or("unknown");

                tracing::info_span!(
                    "http_request",
                    method = %request.method(),
                    uri = %request.uri(),
                    request_id = %request_id,
                )
            }),
        )
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
}

pub async fn run(listener: TcpListener, state: AppState) -> Result<(), std::io::Error> {
    axum::serve(listener, app(state).into_make_service()).await
}

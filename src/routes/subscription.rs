use crate::state::AppState;
use axum::{
    extract::{Form, State},
    http::StatusCode,
};
use chrono::Utc;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    name: String,
    email: String,
}

#[axum::debug_handler]
pub async fn subscribe(
    State(state): State<AppState>,
    Form(sub_info): Form<SubscribeFormData>,
) -> StatusCode {
    let db_pool = &state.db;

    let request_id = Uuid::new_v4();
    // `info_span` creates a span at the info-level
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %sub_info.email,
        subscriber_name = %sub_info.name
    );
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        sub_info.email,
        sub_info.name,
        Utc::now()
    )
    .execute(db_pool)
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            StatusCode::OK
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

use crate::state::AppState;
use axum::{
    extract::{Form, State},
    http::StatusCode,
};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    name: String,
    email: String,
}

#[axum::debug_handler]
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(state, sub_info),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %sub_info.email,
        subscriber_name = %sub_info.name
    )
)]
pub async fn subscribe(
    State(state): State<AppState>,
    Form(sub_info): Form<SubscribeFormData>,
) -> StatusCode {
    let db_pool = &state.db;

    match insert_subscriber(db_pool, &sub_info).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(db_pool, sub_info)
)]
pub async fn insert_subscriber(
    db_pool: &Pool<Postgres>,
    sub_info: &SubscribeFormData,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
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
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

use crate::state::AppState;
use axum::{
    extract::{Form, State},
    http::StatusCode,
};
use chrono::Utc;
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
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

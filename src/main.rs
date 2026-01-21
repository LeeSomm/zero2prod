use std::time::Duration;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::state::AppState;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber(
        "zero2prod".into(),
        "zero2prod=info,tower_http=info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // Panic if we cannot read configuration
    let config = get_configuration().expect("Failed to read configuration.");

    let db = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(10))
        .connect_lazy(config.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres.");

    let state = AppState { db, config };

    let addr = format!(
        "{}:{}",
        state.config.application.host, state.config.application.port
    );
    let listener = TcpListener::bind(addr).await?;

    run(listener, state).await
}

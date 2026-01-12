use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    // Panic if we cannot read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!("0.0.0.0:{}", configuration.application_port);

    run(address).await
}

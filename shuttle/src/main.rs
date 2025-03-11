// shuttle/src/main.rs

// module declarations
mod shuttle_pavex;

// dependencies
use pavex::server::Server;
use server::telemetry::{get_subscriber, init_telemetry};
use shuttle_pavex::PavexService;
use shuttle_runtime::SecretStore;
use std::env::set_var;

#[shuttle_runtime::main]
async fn pavex(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_pavex::ShuttlePavex {
    // this unsafe block is needed as of Rust 2024, as it's been deemed that modifying a running 
    // process is potentially unsafe. This step is necessary because Pavex utilizes environment
    // variables for configuration, and Shuttle does not yet have a container wide way of setting
    // environment variables.
    unsafe {
        set_var(
            "APP_PROFILE",
            secrets.get("APP_PROFILE").unwrap_or("dev".to_string()),
        );
    }

    let _ = dotenvy::dotenv();

    // start up telemetry
    let subscriber = get_subscriber("pavex_shuttle".into(), "info".into(), std::io::stdout);
    init_telemetry(subscriber)?;

    let server = Server::new();

    let shuttle_px = PavexService(server);

    Ok(shuttle_px)
}

mod api;
mod db;
mod settings;

use std::sync::Arc;

use api::subsonic::system::{get_license, ping};
use axum::{Router, routing::get};
use sea_orm::Database;
use settings::Settings;

#[tokio::main]
async fn main() {
    // load server configuration and connect to database
    let settings = Arc::new(Settings::load("./config/development.toml"));
    let host_address = format!("{}:{}", settings.server.host, settings.server.port);
    let db_address = format!("sqlite://{}?mode=rwc", settings.database.file);
    let db = Arc::new(Database::connect(&db_address).await.expect(&format!(
        "[FATAL] Failed to connect to database at {}",
        &db_address
    )));

    // set up API routing and serve
    let router = Router::new()
        .route("/rest/ping", get(ping))
        .route("/rest/ping.view", get(ping))
        .route("/rest/getLicense", get(get_license))
        .route("/rest/getLicense.view", get(get_license))
        .with_state(settings)
        .with_state(db);
    let listener = tokio::net::TcpListener::bind(&host_address)
        .await
        .expect(&format!(
            "[FATAL] Failed to bind listener to {}",
            &host_address
        ));
    axum::serve(listener, router)
        .await
        .expect("[FATAL] Failed to serve application");
}

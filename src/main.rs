mod api;
mod settings;

use api::subsonic::system::{get_license, ping};
use axum::{Router, routing::get};
use settings::Settings;

#[tokio::main]
async fn main() {
    // load server configuration
    let settings: Settings = Settings::load("./config/development.toml");
    let address = format!("{}:{}", settings.server.host, settings.server.port);

    // set up API routing and serve
    let router = Router::new()
        .route("/rest/ping", get(ping))
        .route("/rest/ping.view", get(ping))
        .route("/rest/getLicense", get(get_license))
        .route("/rest/getLicense.view", get(get_license));
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect(&format!("[FATAL] Failed to bind listener to {}", &address));
    axum::serve(listener, router)
        .await
        .expect("[FATAL] Failed to serve application");
}

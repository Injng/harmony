mod settings;

use axum::{Router, routing::get};
use settings::Settings;

async fn hello() -> String {
    String::from("Hello, world!")
}

#[tokio::main]
async fn main() {
    // load server configuration
    let settings: Settings = Settings::load("./config/development.toml");
    let address = format!("{}:{}", settings.server.host, settings.server.port);

    // set up API routing and serve
    let router = Router::new().route("/", get(hello));
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect(&format!("[FATAL] Failed to bind listener to {}", &address));
    axum::serve(listener, router)
        .await
        .expect("[FATAL] Failed to serve application");
}

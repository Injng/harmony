mod api;
mod auth;
mod db;
mod settings;

use std::sync::Arc;

use api::subsonic::{
    system::{api_get_license, api_ping},
    users::api_create_user,
};
use auth::middleware::auth_middleware;
use axum::{Router, middleware, routing::get};
use sea_orm::{Database, DatabaseConnection};
use settings::Settings;

#[derive(Clone)]
struct AppState {
    settings: Arc<Settings>,
    db: Arc<DatabaseConnection>,
}

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
    db.get_schema_registry("harmony::db::*")
        .sync(db.as_ref())
        .await
        .expect("[FATAL] Failed to get schema registry");

    // create shared application state
    let state = AppState { settings, db };

    // set up API routing and serve
    let router = Router::new()
        .route("/rest/ping", get(api_ping))
        .route("/rest/ping.view", get(api_ping))
        .route("/rest/getLicense", get(api_get_license))
        .route("/rest/getLicense.view", get(api_get_license))
        .route("/rest/createUser", get(api_create_user))
        .route("/rest/createUser.view", get(api_create_user))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state);
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

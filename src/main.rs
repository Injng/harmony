use axum::{Router, routing::get};

const ADDRESS: &str = "0.0.0.0:3000";

async fn hello() -> String {
    String::from("Hello, world!")
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(hello));
    let listener = tokio::net::TcpListener::bind(ADDRESS)
        .await
        .expect(&format!("[FATAL] Failed to bind listener to {}", ADDRESS));
    axum::serve(listener, router)
        .await
        .expect("[FATAL] Failed to serve application");
}

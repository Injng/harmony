use axum::Json;
use serde_json::Value;

use crate::api::responses::HarmonyResponse;

pub async fn api_ping() -> Json<Value> {
    let response = HarmonyResponse {
        status: Ok(()),
        with_license: false,
    };
    Json(serde_json::to_value(response).unwrap())
}

pub async fn api_get_license() -> Json<Value> {
    let response = HarmonyResponse {
        status: Ok(()),
        with_license: true,
    };
    Json(serde_json::to_value(response).unwrap())
}

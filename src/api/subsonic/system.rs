use axum::Json;
use serde_json::Value;

use crate::api::responses::SubsonicResponse;

pub async fn ping() -> Json<Value> {
    let response = SubsonicResponse {
        status: Ok(()),
        with_license: false,
    };
    Json(serde_json::to_value(response).unwrap())
}

pub async fn get_license() -> Json<Value> {
    let response = SubsonicResponse {
        status: Ok(()),
        with_license: true,
    };
    Json(serde_json::to_value(response).unwrap())
}

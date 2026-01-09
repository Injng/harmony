use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;
use serde_json::Value;

use crate::{AppState, api::responses::SubsonicResponse, auth::users::auth_create_user};

#[derive(Deserialize)]
pub struct UserParameters {
    username: String,
    password: String,
    email: String,
}

pub async fn api_create_user(
    State(state): State<AppState>,
    Query(params): Query<UserParameters>,
) -> Json<Value> {
    if let Err(e) = auth_create_user(
        &params.username,
        &params.password,
        &params.email,
        &state.settings.key,
        &state.db,
    )
    .await
    {
        let response = SubsonicResponse {
            status: Err(e.to_string()),
            with_license: false,
        };
        return Json(serde_json::to_value(response).unwrap());
    } else {
        let response = SubsonicResponse {
            status: Ok(()),
            with_license: false,
        };
        return Json(serde_json::to_value(response).unwrap());
    }
}

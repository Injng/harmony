use axum::{
    extract::{Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use serde::Deserialize;

use crate::AppState;

use super::auth::auth_check_and_decode_hex;
use super::users::auth_check_user;

#[derive(Deserialize)]
pub struct AuthParameters {
    u: String,
    p: Option<String>,
    t: Option<String>,
    s: Option<String>,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    Query(params): Query<AuthParameters>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // either p or both t and s must be specified
    let token_str: String;
    let salt_str: String;
    if let Some(t) = params.t {
        if let Some(s) = params.s {
            token_str = t;
            salt_str = s;
        } else {
            println!("[ERROR] Token and salt must both be specified");
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        if let Some(p) = params.p {
            if let Ok(dec_password) = auth_check_and_decode_hex(&p) {
                token_str = format!("{:x}", md5::compute(dec_password.as_bytes()));
                salt_str = "".to_string();
            } else {
                println!("[ERROR] Invalid hex-encoded password");
                return Err(StatusCode::UNAUTHORIZED);
            }
        } else {
            println!("[ERROR] Either a password or token and salt must be specified");
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    // check that the user has the correct credentials
    if let Err(e) = auth_check_user(
        &params.u,
        &token_str,
        &salt_str,
        &state.settings.key,
        &state.db,
    )
    .await
    {
        println!("{}", e);
        return Err(StatusCode::UNAUTHORIZED);
    }
    let response = next.run(request).await;
    return Ok(response);
}

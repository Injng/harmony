use axum::{
    Json,
    extract::{Multipart, State},
};
use serde_json::Value;
use uuid::Uuid;

use crate::{AppState, library::artist::artist_set_picture};

use super::responses::HarmonyResponse;

pub async fn api_upload_artist_picture(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Json<Value> {
    // obtain the id and data for the artist picture
    let mut id: Option<Uuid> = None;
    let mut file_data: Option<Vec<u8>> = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("picture") {
            let data = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                Err(e) => {
                    return Json(
                        serde_json::to_value(HarmonyResponse {
                            status: Err(e.to_string()),
                            with_license: false,
                        })
                        .unwrap(),
                    );
                }
            };
            file_data = Some(data);
        } else if field.name() == Some("id") {
            let text = match field.text().await {
                Ok(t) => t,
                Err(e) => {
                    return Json(
                        serde_json::to_value(HarmonyResponse {
                            status: Err(e.to_string()),
                            with_license: false,
                        })
                        .unwrap(),
                    );
                }
            };
            id = match Uuid::parse_str(&text) {
                Ok(u) => Some(u),
                Err(e) => {
                    return Json(
                        serde_json::to_value(HarmonyResponse {
                            status: Err(e.to_string()),
                            with_license: false,
                        })
                        .unwrap(),
                    );
                }
            };
        }
    }

    // try to upload if id and data are available
    if let Some(artist_id) = id {
        if let Some(picture) = file_data {
            match artist_set_picture(artist_id, &state.db, picture).await {
                Ok(()) => {
                    return Json(
                        serde_json::to_value(HarmonyResponse {
                            status: Ok(()),
                            with_license: false,
                        })
                        .unwrap(),
                    );
                }
                Err(e) => {
                    return Json(
                        serde_json::to_value(HarmonyResponse {
                            status: Err(e.to_string()),
                            with_license: false,
                        })
                        .unwrap(),
                    );
                }
            };
        }
    }
    Json(
        serde_json::to_value(HarmonyResponse {
            status: Err("Failed to upload file".to_string()),
            with_license: false,
        })
        .unwrap(),
    )
}

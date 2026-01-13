use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    AppState,
    api::responses::HarmonyResponse,
    library::album::{album_get_newest_list, album_get_random_list},
};

use super::responses::AlbumListResponse;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AlbumListType {
    Random,
    Newest,
}

#[derive(Deserialize)]
pub struct AlbumListParameters {
    #[serde(rename = "type")]
    list_type: AlbumListType,
    size: Option<u32>,
    _offset: Option<u32>,
}

pub async fn api_get_album_list(
    State(state): State<AppState>,
    Query(params): Query<AlbumListParameters>,
) -> Json<Value> {
    // default length is 10
    let mut len = 10;
    if let Some(l) = params.size {
        len = l;
    }

    // return album list based on the type of list requested
    match params.list_type {
        AlbumListType::Random => Json(
            serde_json::to_value(AlbumListResponse {
                harmony: HarmonyResponse {
                    status: Ok(()),
                    with_license: true,
                },
                albums: album_get_random_list(len, &state.db).await,
            })
            .unwrap(),
        ),
        AlbumListType::Newest => Json(
            serde_json::to_value(AlbumListResponse {
                harmony: HarmonyResponse {
                    status: Ok(()),
                    with_license: true,
                },
                albums: album_get_newest_list(len, &state.db).await,
            })
            .unwrap(),
        ),
    }
}

use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    AppState,
    api::responses::{HarmonyResponse, PlaylistListResponse, PlaylistResponse},
    library::playlist::{
        playlist_create, playlist_delete, playlist_get_by_id, playlist_get_list, playlist_update,
    },
};

#[derive(Deserialize)]
pub struct CreatePlaylistParameters {
    name: String,
    description: Option<String>,
}

#[derive(Deserialize)]
pub struct GetPlaylistParameters {
    id: Uuid,
}

#[derive(Deserialize)]
pub struct GetPlaylistsParameters {
    size: Option<u32>,
    _offset: Option<u32>,
}

#[derive(Deserialize)]
pub struct DeletePlaylistParameters {
    id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdatePlaylistParameters {
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
    #[serde(rename = "songIdToAdd")]
    song_add_id: Option<Uuid>,
    #[serde(rename = "songIdToRemove")]
    song_remove_idx: Option<Uuid>,
}

pub async fn api_create_playlist(
    State(state): State<AppState>,
    Query(params): Query<CreatePlaylistParameters>,
) -> Json<Value> {
    match playlist_create(params.name, params.description, &state.db).await {
        Ok(_) => Json(
            serde_json::to_value(HarmonyResponse {
                status: Ok(()),
                with_license: false,
            })
            .unwrap(),
        ),
        Err(e) => Json(
            serde_json::to_value(HarmonyResponse {
                status: Err(e.to_string()),
                with_license: false,
            })
            .unwrap(),
        ),
    }
}

pub async fn api_update_playlist(
    State(state): State<AppState>,
    Query(params): Query<UpdatePlaylistParameters>,
) -> Json<Value> {
    match playlist_update(
        params.id,
        params.name,
        params.description,
        params.song_add_id,
        params.song_remove_idx,
        &state.db,
    )
    .await
    {
        Ok(_) => Json(
            serde_json::to_value(HarmonyResponse {
                status: Ok(()),
                with_license: false,
            })
            .unwrap(),
        ),
        Err(e) => Json(
            serde_json::to_value(HarmonyResponse {
                status: Err(e.to_string()),
                with_license: false,
            })
            .unwrap(),
        ),
    }
}

pub async fn api_get_playlists(
    State(state): State<AppState>,
    Query(params): Query<GetPlaylistsParameters>,
) -> Json<Value> {
    // default length is 10
    let mut len = 10;
    if let Some(l) = params.size {
        len = l;
    }

    Json(
        serde_json::to_value(PlaylistListResponse {
            harmony: HarmonyResponse {
                status: Ok(()),
                with_license: false,
            },
            playlists: playlist_get_list(len, &state.db).await,
        })
        .unwrap(),
    )
}

pub async fn api_get_playlist(
    State(state): State<AppState>,
    Query(params): Query<GetPlaylistParameters>,
) -> Json<Value> {
    match playlist_get_by_id(params.id, &state.db).await {
        Ok(p) => Json(
            serde_json::to_value(PlaylistResponse {
                harmony: HarmonyResponse {
                    status: Ok(()),
                    with_license: false,
                },
                playlist: Some(p),
            })
            .unwrap(),
        ),
        Err(e) => Json(
            serde_json::to_value(PlaylistResponse {
                harmony: HarmonyResponse {
                    status: Err(e.to_string()),
                    with_license: false,
                },
                playlist: None,
            })
            .unwrap(),
        ),
    }
}

pub async fn api_delete_playlist(
    State(state): State<AppState>,
    Query(params): Query<DeletePlaylistParameters>,
) -> Json<Value> {
    match playlist_delete(params.id, &state.db).await {
        Ok(_) => Json(
            serde_json::to_value(HarmonyResponse {
                status: Ok(()),
                with_license: false,
            })
            .unwrap(),
        ),
        Err(e) => Json(
            serde_json::to_value(HarmonyResponse {
                status: Err(e.to_string()),
                with_license: false,
            })
            .unwrap(),
        ),
    }
}

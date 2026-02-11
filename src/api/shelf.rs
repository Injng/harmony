use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    AppState,
    api::responses::{HarmonyResponse, PlaylistListResponse, PlaylistResponse, StarredResponse},
    library::playlist::{
        playlist_create, playlist_delete, playlist_get_by_id, playlist_get_list, playlist_update,
    },
    library::shelf::{
        get_starred_albums, get_starred_books, get_starred_tracks, star_album, star_book,
        star_track, unstar_album, unstar_book, unstar_track,
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

#[derive(Deserialize)]
pub struct StarParameters {
    u: String,
    #[serde(rename = "trackId")]
    track_id: Option<Uuid>,
    #[serde(rename = "albumId")]
    album_id: Option<Uuid>,
    #[serde(rename = "bookId")]
    book_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct GetStarredParameters {
    u: String,
}

pub async fn api_star(
    State(state): State<AppState>,
    Query(params): Query<StarParameters>,
) -> Json<Value> {
    let mut errors: Vec<String> = Vec::new();

    if let Some(track_id) = params.track_id {
        if let Err(e) = star_track(&params.u, track_id, &state.db).await {
            errors.push(e.to_string());
        }
    }

    if let Some(album_id) = params.album_id {
        if let Err(e) = star_album(&params.u, album_id, &state.db).await {
            errors.push(e.to_string());
        }
    }

    if let Some(book_id) = params.book_id {
        if let Err(e) = star_book(&params.u, book_id, &state.db).await {
            errors.push(e.to_string());
        }
    }

    if errors.is_empty() {
        Json(
            serde_json::to_value(HarmonyResponse {
                status: Ok(()),
                with_license: false,
            })
            .unwrap(),
        )
    } else {
        Json(
            serde_json::to_value(HarmonyResponse {
                status: Err(errors.join("; ")),
                with_license: false,
            })
            .unwrap(),
        )
    }
}

pub async fn api_unstar(
    State(state): State<AppState>,
    Query(params): Query<StarParameters>,
) -> Json<Value> {
    let mut errors: Vec<String> = Vec::new();

    if let Some(track_id) = params.track_id {
        if let Err(e) = unstar_track(&params.u, track_id, &state.db).await {
            errors.push(e.to_string());
        }
    }

    if let Some(album_id) = params.album_id {
        if let Err(e) = unstar_album(&params.u, album_id, &state.db).await {
            errors.push(e.to_string());
        }
    }

    if let Some(book_id) = params.book_id {
        if let Err(e) = unstar_book(&params.u, book_id, &state.db).await {
            errors.push(e.to_string());
        }
    }

    if errors.is_empty() {
        Json(
            serde_json::to_value(HarmonyResponse {
                status: Ok(()),
                with_license: false,
            })
            .unwrap(),
        )
    } else {
        Json(
            serde_json::to_value(HarmonyResponse {
                status: Err(errors.join("; ")),
                with_license: false,
            })
            .unwrap(),
        )
    }
}

pub async fn api_get_starred(
    State(state): State<AppState>,
    Query(params): Query<GetStarredParameters>,
) -> Json<Value> {
    let tracks = get_starred_tracks(&params.u, &state.db).await.unwrap_or_default();
    let albums = get_starred_albums(&params.u, &state.db).await.unwrap_or_default();
    let books = get_starred_books(&params.u, &state.db).await.unwrap_or_default();

    Json(
        serde_json::to_value(StarredResponse {
            harmony: HarmonyResponse {
                status: Ok(()),
                with_license: false,
            },
            tracks,
            albums,
            books,
        })
        .unwrap(),
    )
}

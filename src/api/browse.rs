use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    AppState,
    api::responses::{AlbumListResponse, AlbumResponse, HarmonyResponse, TrackResponse},
    library::{
        album::{album_get_by_id, album_get_newest_list, album_get_random_list},
        artist::{artist_get_by_id, artist_get_list},
        book::{book_get_by_id, book_get_list},
        track::track_get_by_id,
    },
};

use super::responses::{ArtistListResponse, ArtistResponse, BookListResponse, BookResponse};

/* -------------------------------------------------------------------------------------------
    MUSIC BROWSING
------------------------------------------------------------------------------------------- */

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

#[derive(Deserialize)]
pub struct ArtistListParameters {
    size: Option<u32>,
    _offset: Option<u32>,
}

#[derive(Deserialize)]
pub struct ArtistParameters {
    id: Uuid,
}

#[derive(Deserialize)]
pub struct AlbumParameters {
    id: Uuid,
}

#[derive(Deserialize)]
pub struct TrackParameters {
    id: Uuid,
}

pub async fn api_get_artist_list(
    State(state): State<AppState>,
    Query(params): Query<ArtistListParameters>,
) -> Json<Value> {
    // default length is 10
    let mut len = 10;
    if let Some(l) = params.size {
        len = l;
    }

    Json(
        serde_json::to_value(ArtistListResponse {
            harmony: HarmonyResponse {
                status: Ok(()),
                with_license: false,
            },
            artists: artist_get_list(len, &state.db).await,
        })
        .unwrap(),
    )
}

pub async fn api_get_artist(
    State(state): State<AppState>,
    Query(params): Query<ArtistParameters>,
) -> Json<Value> {
    match artist_get_by_id(params.id, &state.db).await {
        Ok(a) => Json(
            serde_json::to_value(ArtistResponse {
                harmony: HarmonyResponse {
                    status: Ok(()),
                    with_license: false,
                },
                artist: Some(a),
            })
            .unwrap(),
        ),
        Err(e) => Json(
            serde_json::to_value(ArtistResponse {
                harmony: HarmonyResponse {
                    status: Err(e.to_string()),
                    with_license: false,
                },
                artist: None,
            })
            .unwrap(),
        ),
    }
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
                    with_license: false,
                },
                albums: album_get_random_list(len, &state.db).await,
            })
            .unwrap(),
        ),
        AlbumListType::Newest => Json(
            serde_json::to_value(AlbumListResponse {
                harmony: HarmonyResponse {
                    status: Ok(()),
                    with_license: false,
                },
                albums: album_get_newest_list(len, &state.db).await,
            })
            .unwrap(),
        ),
    }
}

pub async fn api_get_album(
    State(state): State<AppState>,
    Query(params): Query<AlbumParameters>,
) -> Json<Value> {
    match album_get_by_id(params.id, &state.db).await {
        Ok(a) => Json(
            serde_json::to_value(AlbumResponse {
                harmony: HarmonyResponse {
                    status: Ok(()),
                    with_license: false,
                },
                album: Some(a),
            })
            .unwrap(),
        ),
        Err(e) => Json(
            serde_json::to_value(AlbumResponse {
                harmony: HarmonyResponse {
                    status: Err(e.to_string()),
                    with_license: false,
                },
                album: None,
            })
            .unwrap(),
        ),
    }
}

pub async fn api_get_track(
    State(state): State<AppState>,
    Query(params): Query<TrackParameters>,
) -> Json<Value> {
    match track_get_by_id(params.id, &state.db).await {
        Ok(t) => Json(
            serde_json::to_value(TrackResponse {
                harmony: HarmonyResponse {
                    status: Ok(()),
                    with_license: false,
                },
                track: Some(t),
            })
            .unwrap(),
        ),
        Err(e) => Json(
            serde_json::to_value(TrackResponse {
                harmony: HarmonyResponse {
                    status: Err(e.to_string()),
                    with_license: false,
                },
                track: None,
            })
            .unwrap(),
        ),
    }
}

/* ------------------------------------------------------------------------------------------
    BOOK BROWSING
------------------------------------------------------------------------------------------ */

#[derive(Deserialize)]
pub struct BookListParameters {
    size: Option<u32>,
    _offset: Option<u32>,
}

#[derive(Deserialize)]
pub struct BookParameters {
    id: Uuid,
}

pub async fn api_get_books(
    State(state): State<AppState>,
    Query(params): Query<BookListParameters>,
) -> Json<Value> {
    // default length is 10
    let mut len = 10;
    if let Some(l) = params.size {
        len = l;
    }

    Json(
        serde_json::to_value(BookListResponse {
            harmony: HarmonyResponse {
                status: Ok(()),
                with_license: false,
            },
            books: book_get_list(len, &state.db).await,
        })
        .unwrap(),
    )
}

pub async fn api_get_book(
    State(state): State<AppState>,
    Query(params): Query<BookParameters>,
) -> Json<Value> {
    match book_get_by_id(params.id, &state.db).await {
        Ok(a) => Json(
            serde_json::to_value(BookResponse {
                harmony: HarmonyResponse {
                    status: Ok(()),
                    with_license: false,
                },
                book: Some(a),
            })
            .unwrap(),
        ),
        Err(e) => Json(
            serde_json::to_value(BookResponse {
                harmony: HarmonyResponse {
                    status: Err(e.to_string()),
                    with_license: false,
                },
                book: None,
            })
            .unwrap(),
        ),
    }
}

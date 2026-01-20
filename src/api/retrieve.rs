use axum::{
    body::Body,
    extract::{Query, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::{
    AppState,
    library::{book::book_get_by_id, track::track_get_by_id},
};

#[derive(Deserialize)]
pub struct RetrieveParameters {
    id: Uuid,
}

pub async fn api_stream_track(
    State(state): State<AppState>,
    Query(params): Query<RetrieveParameters>,
) -> Result<impl IntoResponse, StatusCode> {
    // get track with file info
    let track = track_get_by_id(params.id, &state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // get file path from the track's file relation
    let file_path = track
        .file
        .as_ref()
        .ok_or(StatusCode::NOT_FOUND)?
        .path
        .clone();

    // open the file and create a stream
    let file = File::open(&file_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "audio/flac")
        .body(body)
        .unwrap())
}

pub async fn api_fetch_book(
    State(state): State<AppState>,
    Query(params): Query<RetrieveParameters>,
) -> Result<impl IntoResponse, StatusCode> {
    // get book with file info
    let book = book_get_by_id(params.id, &state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // get file path from the book's file relation
    let file_path = book
        .file
        .as_ref()
        .ok_or(StatusCode::NOT_FOUND)?
        .path
        .clone();

    // open the file and create a stream
    let file = File::open(&file_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "application/epub+zip")
        .body(body)
        .unwrap())
}

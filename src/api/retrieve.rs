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

use crate::{AppState, library::track::track_get_by_id};

#[derive(Deserialize)]
pub struct StreamParameters {
    id: Uuid,
}

pub async fn api_stream_track(
    State(state): State<AppState>,
    Query(params): Query<StreamParameters>,
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

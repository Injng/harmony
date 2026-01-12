use std::{ffi::OsStr, fs, path::Path};

use anyhow::Result;
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{ColumnTrait, DatabaseConnection, Set};
use walkdir::WalkDir;

use crate::db::{album, artist, track};
use crate::library::album::album_find;
use crate::library::artist::artist_insert;
use crate::{
    db::file::{self, Entity as File},
    format::flac::parse_flac_file,
};

use super::track::Track;

async fn scan_flac(path: &Path, db: &DatabaseConnection) -> Result<()> {
    // check if file exists in database
    let file: Option<file::Model> = File::find()
        .filter(file::Column::Path.eq(path.display().to_string()))
        .one(db)
        .await?;

    // if file exists, only continue if last modified is more recent
    let modified: DateTime<Utc> = fs::metadata(path)?.modified()?.into();
    if let Some(f) = &file {
        if modified <= f.last_modified {
            return Ok(());
        }
    }

    // extract useful metadata from file
    let metadata = parse_flac_file(path)?;
    let album_name: String = metadata.get_album_name()?;
    let track_name: String = metadata.get_track_name()?;
    let artists: Vec<String> = metadata.get_artists()?;
    let album_artists: Option<Vec<String>> = metadata.get_album_artists();
    let musicbrainz_album_id: Option<String> = metadata.get_musicbrainz_album_id();

    // turn list of artists into active models
    let mut artist_models: Vec<artist::ActiveModel> = Vec::new();
    for artist in &artists {
        artist_models.push(artist_insert(artist, db).await);
    }

    // check if album exists in database already
    let album = album_find(
        &album_name,
        artists,
        album_artists.clone(),
        musicbrainz_album_id,
        db,
    )
    .await;

    let track_id = Uuid::new_v4();
    if let Some(album_id) = album {
        // insert new track with existing album into the database
        let mut track = track::ActiveModel::builder()
            .set_id(track_id)
            .set_title(track_name)
            .set_album_id(album_id);
        for artist in artist_models {
            track = track.add_artist(artist);
        }
        let _ = track.insert(db).await?;
    } else {
        // insert new album into the database
        let album_id = Uuid::new_v4();
        let mut album = album::ActiveModel::builder()
            .set_id(album_id)
            .set_name(album_name);
        if let Some(aa) = album_artists {
            for artist in &aa {
                album = album.add_artist(artist_insert(artist, db).await);
            }
        }
        let _ = album.insert(db).await?;

        // insert new track into the database
        let mut track = track::ActiveModel::builder()
            .set_id(track_id)
            .set_title(track_name)
            .set_album_id(album_id);
        for artist in artist_models {
            track = track.add_artist(artist);
        }
        let _ = track.insert(db).await?;
    }

    // update file in the database
    if let Some(f) = file {
        let mut f: file::ActiveModel = f.into();
        f.last_modified = Set(modified);
        let _ = f.update(db).await?;
    } else {
        let f = file::ActiveModel::builder()
            .set_id(Uuid::new_v4())
            .set_path(path.display().to_string())
            .set_last_modified(modified)
            .set_track_id(track_id);
        let _ = f.insert(db).await?;
    }

    return Ok(());
}

pub async fn scan(path: &str, db: &DatabaseConnection) -> Result<()> {
    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type().is_file() && path.extension() == Some(OsStr::new("flac")) {
            let _ = scan_flac(path, db).await?;
        }
    }
    Ok(())
}

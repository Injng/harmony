use std::{fs, path::Path};

use anyhow::Result;
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{ColumnTrait, DatabaseConnection, ModelTrait, Set};
use walkdir::WalkDir;

use crate::db::{album, artist, book, track};
use crate::format::epub::parse_epub_file;
use crate::format::flac::FlacPictureType;
use crate::library::album::album_find;
use crate::library::artist::artist_insert;
use crate::{
    db::file::{self, Entity as File},
    format::flac::parse_flac_file,
};

use super::track::TrackMetadata;

async fn scan_epub(path: &Path, db: &DatabaseConnection) -> Result<()> {
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
    let metadata = parse_epub_file(path)?;
    let mut artists: Vec<String> = Vec::new();
    if let Some(a) = metadata.creator {
        artists.push(a);
    }

    // turn list of artists into active models
    let mut artist_models: Vec<artist::ActiveModel> = Vec::new();
    for artist in &artists {
        artist_models.push(artist_insert(artist, db).await);
    }

    // check if file has an existing book (update case) or needs new book (insert case)
    let existing_book_id = file.as_ref().and_then(|f| f.track_id);
    let book_id = if let Some(book_id) = existing_book_id {
        // update existing book
        let mut book = book::ActiveModel::builder()
            .set_id(book_id)
            .set_title(metadata.title.unwrap_or("".to_owned()))
            .set_picture(metadata.cover);
        for artist in artist_models {
            book = book.add_artist(artist);
        }
        let _ = book.save(db).await?;
        book_id
    } else {
        // insert new book
        let book_id = Uuid::new_v4();
        let mut book = book::ActiveModel::builder()
            .set_id(book_id)
            .set_title(metadata.title.unwrap_or("".to_owned()))
            .set_picture(metadata.cover);
        for artist in artist_models {
            book = book.add_artist(artist);
        }
        let _ = book.insert(db).await?;
        book_id
    };

    // update or create file in the database (must do this last)
    if let Some(f) = file {
        let mut f: file::ActiveModel = f.into();
        f.last_modified = Set(modified);
        let _ = f.update(db).await?;
    } else {
        let f = file::ActiveModel::builder()
            .set_id(Uuid::new_v4())
            .set_path(path.display().to_string())
            .set_last_modified(modified)
            .set_book_id(book_id);
        let _ = f.insert(db).await?;
    }

    return Ok(());
}

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
    let picture: Option<Vec<u8>> = metadata.get_picture_data(FlacPictureType::FrontCover);

    // check if album exists in database already
    let album_id = match album_find(
        &album_name,
        &artists,
        album_artists.clone(),
        musicbrainz_album_id.clone(),
        db,
    )
    .await
    {
        Some(id) => id,
        None => {
            // insert new album into the database
            let album_id = Uuid::new_v4();
            let mut album = album::ActiveModel::builder()
                .set_id(album_id)
                .set_name(album_name.trim())
                .set_musicbrainz_id(musicbrainz_album_id)
                .set_last_modified(modified);
            if let Some(aa) = album_artists {
                for artist in &aa {
                    album = album.add_artist(artist_insert(artist, db).await);
                }
            }
            let _ = album.insert(db).await?;
            album_id
        }
    };

    // turn list of artists into active models
    let mut artist_models: Vec<artist::ActiveModel> = Vec::new();
    for artist in &artists {
        artist_models.push(artist_insert(artist, db).await);
    }

    // check if file has an existing track (update case) or needs new track (insert case)
    let existing_track_id = file.as_ref().and_then(|f| f.track_id);
    let track_id = if let Some(track_id) = existing_track_id {
        // update existing track
        let mut track = track::ActiveModel::builder()
            .set_id(track_id)
            .set_title(track_name.trim())
            .set_picture(picture)
            .set_album_id(album_id);
        for artist in artist_models {
            track = track.add_artist(artist);
        }
        let _ = track.save(db).await?;
        track_id
    } else {
        // insert new track
        let track_id = Uuid::new_v4();
        let mut track = track::ActiveModel::builder()
            .set_id(track_id)
            .set_title(track_name.trim())
            .set_picture(picture)
            .set_album_id(album_id);
        for artist in artist_models {
            track = track.add_artist(artist);
        }
        let _ = track.insert(db).await?;
        track_id
    };

    // update or create file in the database (must do this last)
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

async fn scan_cleanup(path: &str, db: &DatabaseConnection) -> Result<()> {
    // find all the files in the library that are in the database
    let files = File::find()
        .filter(file::Column::Path.starts_with(path))
        .all(db)
        .await?;

    // delete file records if the file no longer exists
    for f in files {
        if !Path::new(&f.path).exists() {
            let track_id = f.track_id;
            f.delete(db).await?;
            if let Some(track_id) = track_id {
                if let Some(t) = track::Entity::find_by_id(track_id).one(db).await? {
                    t.delete(db).await?;
                }
            }
        }
    }

    // delete orphaned albums (albums with no tracks)
    let albums = album::Entity::find().all(db).await?;
    for a in albums {
        let track_count = track::Entity::find()
            .filter(track::Column::AlbumId.eq(a.id))
            .count(db)
            .await?;
        if track_count == 0 {
            a.delete(db).await?;
        }
    }

    Ok(())
}

pub async fn scan(path: &str, db: &DatabaseConnection) -> Result<()> {
    scan_cleanup(&path, db).await?;
    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path = entry.path();
        if !entry.file_type().is_file() {
            continue;
        }
        match path.extension().and_then(|s| s.to_str()) {
            Some("flac") => scan_flac(path, db).await?,
            Some("epub") => scan_epub(path, db).await?,
            _ => continue,
        };
    }
    Ok(())
}

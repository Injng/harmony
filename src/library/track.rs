use anyhow::{Result, anyhow};
use sea_orm::{DatabaseConnection, EntityLoaderTrait};
use uuid::Uuid;

use crate::{
    db::{
        artist::Entity as Artist,
        file::Entity as File,
        track::{self, Entity as Track},
    },
    format::flac::FlacPictureType,
};

pub trait TrackMetadata {
    // required metadata fields
    fn get_album_name(&self) -> Result<String>;
    fn get_track_name(&self) -> Result<String>;
    fn get_artists(&self) -> Result<Vec<String>>;

    // optional metadata fields
    fn get_album_artists(&self) -> Option<Vec<String>>;
    fn get_musicbrainz_album_id(&self) -> Option<String>;
    fn get_picture_data(&self, priority: FlacPictureType) -> Option<Vec<u8>>;
}

/// Gets a specific track from the database.
pub async fn track_get_by_id(id: Uuid, db: &DatabaseConnection) -> Result<track::ModelEx> {
    if let Ok(Some(t)) = Track::load()
        .with(Artist)
        .with(File)
        .filter_by_id(id)
        .one(db)
        .await
    {
        return Ok(t);
    } else {
        return Err(anyhow!("[ERROR] Track not found in database"));
    }
}

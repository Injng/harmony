use anyhow::{Result, anyhow};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityLoaderTrait, EntityTrait, Order, QueryFilter,
    QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::db::{
    playlist::{self, Entity as Playlist},
    track::Entity as Track,
    track_playlists::{self, Entity as TrackPlaylist},
};

use super::track::track_get_by_id;

/// Creates a new playlist with the name and description provided.
pub async fn playlist_create(
    name: String,
    description: Option<String>,
    db: &DatabaseConnection,
) -> Result<()> {
    let mut playlist = playlist::ActiveModel::builder()
        .set_id(Uuid::new_v4())
        .set_name(name.trim());
    if let Some(d) = description {
        playlist = playlist.set_description(d);
    }
    let _ = playlist.insert(db).await?;
    Ok(())
}

/// Updates a playlist based on the provided parameters.
pub async fn playlist_update(
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
    song_add_id: Option<Uuid>,
    song_remove_id: Option<Uuid>,
    db: &DatabaseConnection,
) -> Result<()> {
    let mut playlist = playlist::ActiveModel::builder().set_id(id);

    // update the name if a name is provided
    if let Some(n) = name {
        playlist = playlist.set_name(n);
    }

    // update the description if a description is provided
    if let Some(d) = description {
        playlist = playlist.set_description(d);
    }

    // insert a track to the top of the playlist if an id is provided
    if let Some(add_id) = song_add_id {
        let track = track_get_by_id(add_id, db).await?;
        playlist = playlist.add_track(track);
    }

    // delete a track if an index to delete is given
    if let Some(song_id) = song_remove_id {
        TrackPlaylist::delete_many()
            .filter(track_playlists::Column::PlaylistId.eq(id))
            .filter(track_playlists::Column::TrackId.eq(song_id))
            .exec(db)
            .await?;
    }
    let _ = playlist.save(db).await?;
    Ok(())
}

/// Returns a sorted list of all the playlists in the database.
pub async fn playlist_get_list(len: u32, db: &DatabaseConnection) -> Vec<playlist::Model> {
    if let Ok(m) = Playlist::find()
        .order_by(playlist::Column::Name, Order::Asc)
        .limit(len as u64)
        .all(db)
        .await
    {
        return m;
    } else {
        return Vec::new();
    }
}

/// Gets a specific playlist from the database.
pub async fn playlist_get_by_id(id: Uuid, db: &DatabaseConnection) -> Result<playlist::ModelEx> {
    if let Ok(Some(a)) = Playlist::load().with(Track).filter_by_id(id).one(db).await {
        return Ok(a);
    } else {
        return Err(anyhow!("[ERROR] Playlist not found in database"));
    }
}

/// Deletes a specific playlist from the database
pub async fn playlist_delete(id: Uuid, db: &DatabaseConnection) -> Result<()> {
    Playlist::delete_by_id(id).exec(db).await?;
    Ok(())
}

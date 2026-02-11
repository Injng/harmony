use anyhow::{Result, anyhow};
use sea_orm::entity::prelude::*;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityLoaderTrait, EntityTrait, Order, QueryOrder,
    QuerySelect, Set,
};
use uuid::Uuid;

use crate::db::{
    album::Entity as Album,
    artist::{self, Entity as Artist},
    track::Entity as Track,
};

/// Checks if an artist already exists in the database by matching the given metadata.
/// A match is found if there is an artist with the same artist_name.
async fn artist_find(artist_name: &str, db: &DatabaseConnection) -> Option<artist::ActiveModel> {
    if let Ok(Some(m)) = Artist::find()
        .filter(artist::Column::Name.eq(artist_name))
        .one(db)
        .await
    {
        return Some(m.into());
    }
    return None;
}

/// Returns the active model of either the artist that is already existing in the database,
/// or of a new artist if none matches within the database. The new artist is not yet inserted
/// into the database.
pub async fn artist_insert(name: &str, db: &DatabaseConnection) -> artist::ActiveModel {
    if let Some(m) = artist_find(name.trim(), db).await {
        return m;
    } else {
        return artist::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name.trim().to_owned()),
            picture: Set(None),
        };
    }
}

/// Returns a sorted list of all the artists in the database.
pub async fn artist_get_list(len: u32, db: &DatabaseConnection) -> Vec<artist::Model> {
    if let Ok(m) = Artist::find()
        .order_by(artist::Column::Name, Order::Asc)
        .limit(len as u64)
        .all(db)
        .await
    {
        return m;
    } else {
        return Vec::new();
    }
}

/// Gets a specific artist from the database.
pub async fn artist_get_by_id(id: Uuid, db: &DatabaseConnection) -> Result<artist::ModelEx> {
    if let Ok(Some(a)) = Artist::load()
        .with(Album)
        .with(Track)
        .filter_by_id(id)
        .one(db)
        .await
    {
        return Ok(a);
    } else {
        return Err(anyhow!("[ERROR] Artist not found in database"));
    }
}

/// Sets the picture of a specific artist in the database.
pub async fn artist_set_picture(
    id: Uuid,
    db: &DatabaseConnection,
    artist_picture: Vec<u8>,
) -> Result<()> {
    let mut artist = artist::ActiveModel::builder().set_id(id);
    artist = artist.set_picture(Some(artist_picture));
    let _ = artist.save(db).await?;
    Ok(())
}

use rand::rng;
use rand::seq::IteratorRandom;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityLoaderTrait, Order, QueryOrder, prelude::*};
use uuid::Uuid;

use crate::db::{
    album::{self, Entity as Album, ModelEx},
    artist::Entity as Artist,
};

/// Checks if an album is a match with the given metadata. Assumes the names are the same.
/// A match is found only in the following cases:
///     (1) If the album_artists are present, then:
///         (i) If both the album_name and album_artists are the same
///     (2) If only artists are present, then:
///         (i) If the album_name is the same, and at least one artist is in album_artists
pub async fn album_match(
    artists: &Vec<String>,
    album_artists: &Option<Vec<String>>,
    album: &ModelEx,
) -> bool {
    // if the album_artists are present, then it is a match only if they are the same
    if let Some(aa) = album_artists {
        if aa.len() != album.artists.len() {
            return false;
        }
        return aa
            .iter()
            .all(|name| album.artists.iter().any(|artist| artist.name == *name));
    }

    // otherwise, check if at least one artist is in album_artists on the database
    return artists
        .iter()
        .any(|name| album.artists.iter().any(|artist| artist.name == *name));
}

/// Checks if an album already exists in the database by matching the given metadata.
/// A match is found by first checking if the same musicbrainz_album_id exists on the
/// database, if it was provided to the function. Otherwise, it attempts to match each
/// album on the database with the same album_name using the function album_match().
pub async fn album_find(
    album_name: &str,
    artists: &Vec<String>,
    album_artists: Option<Vec<String>>,
    musicbrainz_album_id: Option<String>,
    db: &DatabaseConnection,
) -> Option<Uuid> {
    // check case where musicbrainz_album_id exists both in the file and on the database
    if let Some(id) = musicbrainz_album_id {
        if let Ok(Some(m)) = Album::find()
            .filter(album::Column::MusicbrainzId.eq(id))
            .one(db)
            .await
        {
            return Some(m.id);
        }
    }

    // otherwise, find all albums in the database with the same album name
    if let Ok(m) = Album::load()
        .filter(album::Column::Name.eq(album_name))
        .with(Artist)
        .all(db)
        .await
    {
        for album in m {
            if album_match(&artists, &album_artists, &album).await {
                return Some(album.id);
            }
        }
    }

    return None;
}

/// Gets a list of random albums from the database.
pub async fn album_get_random_list(len: u32, db: &DatabaseConnection) -> Vec<album::ModelEx> {
    if let Ok(m) = Album::load().with(Artist).all(db).await {
        let mut r = rng();
        return m.into_iter().choose_multiple(&mut r, len as usize);
    } else {
        return Vec::new();
    }
}

pub async fn album_get_newest_list(len: u32, db: &DatabaseConnection) -> Vec<album::ModelEx> {
    let paginator = Album::load()
        .with(Artist)
        .order_by(album::Column::LastModified, Order::Desc)
        .paginate(db, len as u64);
    if let Ok(m) = paginator.fetch().await {
        return m;
    } else {
        return Vec::new();
    }
}

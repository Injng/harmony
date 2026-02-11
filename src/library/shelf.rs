use anyhow::{Result, anyhow};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::db::{
    album::{self, Entity as Album},
    book::{self, Entity as Book},
    starred_albums::{self, Entity as StarredAlbum},
    starred_books::{self, Entity as StarredBook},
    starred_tracks::{self, Entity as StarredTrack},
    track::{self, Entity as Track},
    user::{self, Entity as User},
};

/// Gets a user's ID from their username.
async fn get_user_id(username: &str, db: &DatabaseConnection) -> Result<Uuid> {
    if let Some(user) = User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await?
    {
        Ok(user.id)
    } else {
        Err(anyhow!("[ERROR] User not found"))
    }
}

/// Stars a track for a user.
pub async fn star_track(username: &str, track_id: Uuid, db: &DatabaseConnection) -> Result<()> {
    let user_id = get_user_id(username, db).await?;
    if Track::find_by_id(track_id).one(db).await?.is_none() {
        return Err(anyhow!("[ERROR] Track not found"));
    }
    if StarredTrack::find_by_id((user_id, track_id))
        .one(db)
        .await?
        .is_some()
    {
        return Ok(());
    }

    let starred = starred_tracks::ActiveModel::builder()
        .set_user_id(user_id)
        .set_track_id(track_id);
    starred.insert(db).await?;
    Ok(())
}

/// Unstars a track for a user.
pub async fn unstar_track(username: &str, track_id: Uuid, db: &DatabaseConnection) -> Result<()> {
    let user_id = get_user_id(username, db).await?;

    StarredTrack::delete_by_id((user_id, track_id))
        .exec(db)
        .await?;
    Ok(())
}

/// Stars an album for a user.
pub async fn star_album(username: &str, album_id: Uuid, db: &DatabaseConnection) -> Result<()> {
    let user_id = get_user_id(username, db).await?;

    // check if album exists
    if Album::find_by_id(album_id).one(db).await?.is_none() {
        return Err(anyhow!("[ERROR] Album not found"));
    }

    // check if already starred
    if StarredAlbum::find_by_id((user_id, album_id))
        .one(db)
        .await?
        .is_some()
    {
        return Ok(());
    }

    let starred = starred_albums::ActiveModel::builder()
        .set_user_id(user_id)
        .set_album_id(album_id);
    starred.insert(db).await?;
    Ok(())
}

/// Unstars an album for a user.
pub async fn unstar_album(username: &str, album_id: Uuid, db: &DatabaseConnection) -> Result<()> {
    let user_id = get_user_id(username, db).await?;

    StarredAlbum::delete_by_id((user_id, album_id))
        .exec(db)
        .await?;
    Ok(())
}

/// Stars a book for a user.
pub async fn star_book(username: &str, book_id: Uuid, db: &DatabaseConnection) -> Result<()> {
    let user_id = get_user_id(username, db).await?;

    // check if book exists
    if Book::find_by_id(book_id).one(db).await?.is_none() {
        return Err(anyhow!("[ERROR] Book not found"));
    }

    // check if already starred
    if StarredBook::find_by_id((user_id, book_id))
        .one(db)
        .await?
        .is_some()
    {
        return Ok(());
    }

    let starred = starred_books::ActiveModel::builder()
        .set_user_id(user_id)
        .set_book_id(book_id);
    starred.insert(db).await?;
    Ok(())
}

/// Unstars a book for a user.
pub async fn unstar_book(username: &str, book_id: Uuid, db: &DatabaseConnection) -> Result<()> {
    let user_id = get_user_id(username, db).await?;

    StarredBook::delete_by_id((user_id, book_id))
        .exec(db)
        .await?;
    Ok(())
}

/// Gets all starred tracks for a user.
pub async fn get_starred_tracks(
    username: &str,
    db: &DatabaseConnection,
) -> Result<Vec<track::Model>> {
    let user_id = get_user_id(username, db).await?;

    let starred = StarredTrack::find()
        .filter(starred_tracks::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    let track_ids: Vec<Uuid> = starred.iter().map(|s| s.track_id).collect();
    let tracks = Track::find()
        .filter(track::Column::Id.is_in(track_ids))
        .all(db)
        .await?;

    Ok(tracks)
}

/// Gets all starred albums for a user.
pub async fn get_starred_albums(
    username: &str,
    db: &DatabaseConnection,
) -> Result<Vec<album::Model>> {
    let user_id = get_user_id(username, db).await?;

    let starred = StarredAlbum::find()
        .filter(starred_albums::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    let album_ids: Vec<Uuid> = starred.iter().map(|s| s.album_id).collect();
    let albums = Album::find()
        .filter(album::Column::Id.is_in(album_ids))
        .all(db)
        .await?;

    Ok(albums)
}

/// Gets all starred books for a user.
pub async fn get_starred_books(
    username: &str,
    db: &DatabaseConnection,
) -> Result<Vec<book::Model>> {
    let user_id = get_user_id(username, db).await?;

    let starred = StarredBook::find()
        .filter(starred_books::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    let book_ids: Vec<Uuid> = starred.iter().map(|s| s.book_id).collect();
    let books = Book::find()
        .filter(book::Column::Id.is_in(book_ids))
        .all(db)
        .await?;

    Ok(books)
}

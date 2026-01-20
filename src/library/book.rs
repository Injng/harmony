use anyhow::{Result, anyhow};
use sea_orm::{DatabaseConnection, EntityLoaderTrait, Order, QueryOrder};
use uuid::Uuid;

use crate::db::{
    artist::Entity as Artist,
    book::{self, Entity as Book},
};

/// Returns a sorted list of all the books in the database.
pub async fn book_get_list(len: u32, db: &DatabaseConnection) -> Vec<book::ModelEx> {
    if let Ok(m) = Book::load()
        .with(Artist)
        .order_by(book::Column::Title, Order::Asc)
        .paginate(db, len as u64)
        .fetch()
        .await
    {
        return m;
    } else {
        return Vec::new();
    }
}

/// Gets a specific book from the database.
pub async fn book_get_by_id(id: Uuid, db: &DatabaseConnection) -> Result<book::ModelEx> {
    if let Ok(Some(a)) = Book::load().with(Artist).filter_by_id(id).one(db).await {
        return Ok(a);
    } else {
        return Err(anyhow!("[ERROR] Book not found in database"));
    }
}

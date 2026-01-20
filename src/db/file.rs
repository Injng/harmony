use chrono::{DateTime, Utc};
use sea_orm::Set;
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub path: String,
    pub last_modified: DateTime<Utc>,
    #[sea_orm(unique)]
    pub track_id: Option<Uuid>,
    #[sea_orm(belongs_to, from = "track_id", to = "id", on_delete = "Cascade")]
    pub track: HasOne<super::track::Entity>,
    #[sea_orm(unique)]
    pub book_id: Option<Uuid>,
    #[sea_orm(belongs_to, from = "book_id", to = "id", on_delete = "Cascade")]
    pub book: HasOne<super::book::Entity>,
}

impl ActiveModelBehavior for ActiveModel {
    fn after_save<'life0, 'async_trait, C>(
        model: Model,
        db: &'life0 C,
        _insert: bool,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Model, DbErr>> + Send + 'async_trait>,
    >
    where
        C: ConnectionTrait + 'async_trait,
        Self: Send + 'async_trait,
        'life0: 'async_trait,
    {
        Box::pin(async move {
            // propagate last_modified to the album via track
            if let Some(track_id) = model.track_id {
                if let Some(track) = super::track::Entity::find_by_id(track_id).one(db).await? {
                    if let Some(album) = super::album::Entity::find_by_id(track.album_id)
                        .one(db)
                        .await?
                    {
                        let mut album: super::album::ActiveModel = album.into();
                        album.last_modified = Set(model.last_modified);
                        album.update(db).await?;
                    }
                }
            }
            Ok(model)
        })
    }
}

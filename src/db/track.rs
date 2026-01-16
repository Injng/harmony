use chrono::{DateTime, Utc};
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tracks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    pub picture: Option<Vec<u8>>,
    #[sea_orm(default_value = 0)]
    pub plays: u32,
    pub last_played: Option<DateTime<Utc>>,
    pub album_id: Uuid,
    #[sea_orm(has_one)]
    pub file: HasOne<super::file::Entity>,
    #[sea_orm(belongs_to, from = "album_id", to = "id")]
    pub album: HasOne<super::album::Entity>,
    #[sea_orm(has_many, via = "track_artists")]
    pub artists: HasMany<super::artist::Entity>,
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
            // propagate the track picture to the album
            if let Some(album) = super::album::Entity::find_by_id(model.album_id)
                .one(db)
                .await?
            {
                let mut album: super::album::ActiveModel = album.into();
                album.picture = Set(model.picture.clone());
                album.update(db).await?;
            }
            Ok(model)
        })
    }
}

impl Serialize for ModelEx {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Track", 6)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("plays", &self.plays)?;
        state.serialize_field("lastPlayed", &self.last_played)?;
        state.serialize_field("albumId", &self.album_id.to_string())?;
        state.serialize_field(
            "artists",
            &self
                .artists
                .iter()
                .map(|a| a.name.clone())
                .collect::<Vec<_>>(),
        )?;
        state.end()
    }
}

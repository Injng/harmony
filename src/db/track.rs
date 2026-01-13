use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tracks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    #[sea_orm(default_value = "0")]
    pub plays: u32,
    #[sea_orm(default_value = "None")]
    pub last_played: Option<DateTime<Utc>>,
    pub album_id: Uuid,
    #[sea_orm(has_one)]
    pub file: HasOne<super::file::Entity>,
    #[sea_orm(belongs_to, from = "album_id", to = "id")]
    pub album: HasOne<super::album::Entity>,
    #[sea_orm(has_many, via = "track_artists")]
    pub artists: HasMany<super::artist::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "artists")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    #[sea_orm(has_many, via = "album_artists")]
    pub album: HasMany<super::album::Entity>,
    #[sea_orm(has_many, via = "track_artists")]
    pub track: HasMany<super::track::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

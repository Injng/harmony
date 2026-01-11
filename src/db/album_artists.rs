use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "album_artists")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub album_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub artist_id: i32,
    #[sea_orm(belongs_to, from = "album_id", to = "id")]
    pub album: Option<super::album::Entity>,
    #[sea_orm(belongs_to, from = "artist_id", to = "id")]
    pub artist: Option<super::artist::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

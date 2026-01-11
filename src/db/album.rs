use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "albums")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    #[sea_orm(has_many, via = "album_artists")]
    pub artists: HasMany<super::artist::Entity>,
    #[sea_orm(has_many)]
    pub tracks: HasMany<super::track::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

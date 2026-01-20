use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "book_artists")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub book_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub artist_id: Uuid,
    #[sea_orm(belongs_to, from = "book_id", to = "id", on_delete = "Cascade")]
    pub track: Option<super::book::Entity>,
    #[sea_orm(belongs_to, from = "artist_id", to = "id", on_delete = "Cascade")]
    pub artist: Option<super::artist::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

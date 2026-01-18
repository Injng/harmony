use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "track_playlists")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub track_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub playlist_id: Uuid,
    #[sea_orm(belongs_to, from = "track_id", to = "id", on_delete = "Cascade")]
    pub track: Option<super::track::Entity>,
    #[sea_orm(belongs_to, from = "playlist_id", to = "id", on_delete = "Cascade")]
    pub playlist: Option<super::playlist::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

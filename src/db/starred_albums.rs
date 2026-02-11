use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "starred_albums")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub album_id: Uuid,
    #[sea_orm(belongs_to, from = "user_id", to = "id", on_delete = "Cascade")]
    pub user: Option<super::user::Entity>,
    #[sea_orm(belongs_to, from = "album_id", to = "id", on_delete = "Cascade")]
    pub album: Option<super::album::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

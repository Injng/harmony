use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "starred_books")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub book_id: Uuid,
    #[sea_orm(belongs_to, from = "user_id", to = "id", on_delete = "Cascade")]
    pub user: Option<super::user::Entity>,
    #[sea_orm(belongs_to, from = "book_id", to = "id", on_delete = "Cascade")]
    pub book: Option<super::book::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

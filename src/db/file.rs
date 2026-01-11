use chrono::{DateTime, Utc};
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
    #[sea_orm(belongs_to, from = "track_id", to = "id")]
    pub track: HasOne<super::track::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

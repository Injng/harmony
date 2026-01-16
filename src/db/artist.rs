use sea_orm::entity::prelude::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

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

impl Serialize for Model {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Artist", 2)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

use base64::{Engine, engine::general_purpose};
use sea_orm::entity::prelude::*;
use serde::{Serialize, Serializer, ser::SerializeStruct};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "books")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    pub picture: Option<Vec<u8>>,
    #[sea_orm(has_one)]
    pub file: HasOne<super::file::Entity>,
    #[sea_orm(has_many, via = "book_artists")]
    pub artists: HasMany<super::artist::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

impl Serialize for ModelEx {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Book", 6)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("title", &self.title)?;
        if let Some(p) = &self.picture {
            state.serialize_field("picture", &Some(general_purpose::STANDARD.encode(p)))?;
        } else {
            state.serialize_field("picture", &None::<String>)?;
        }
        state.serialize_field(
            "artists",
            &self
                .artists
                .iter()
                .map(|a| a.name.clone())
                .collect::<Vec<_>>(),
        )?;
        state.end()
    }
}

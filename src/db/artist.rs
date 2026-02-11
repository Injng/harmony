use base64::{Engine, engine::general_purpose};
use sea_orm::entity::prelude::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "artists")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub picture: Option<Vec<u8>>,
    #[sea_orm(has_many, via = "album_artists")]
    pub albums: HasMany<super::album::Entity>,
    #[sea_orm(has_many, via = "track_artists")]
    pub tracks: HasMany<super::track::Entity>,
    #[sea_orm(has_many, via = "book_artists")]
    pub books: HasMany<super::book::Entity>,
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
        if let Some(p) = &self.picture {
            state.serialize_field("picture", &Some(general_purpose::STANDARD.encode(p)))?;
        } else {
            state.serialize_field("picture", &None::<String>)?;
        }
        state.end()
    }
}

impl Serialize for ModelEx {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Artist", 2)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("albums", &self.albums)?;
        state.serialize_field("tracks", &self.tracks)?;
        state.serialize_field("books", &self.books)?;
        state.end()
    }
}

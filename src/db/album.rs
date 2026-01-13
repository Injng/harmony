use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "albums")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    #[sea_orm(default_value = "0")]
    pub plays: u32,
    #[sea_orm(default_value = "None")]
    pub last_played: Option<DateTime<Utc>>,
    pub last_modified: DateTime<Utc>,
    #[sea_orm(nullable)]
    pub musicbrainz_id: Option<String>,
    #[sea_orm(has_many, via = "album_artists")]
    pub artists: HasMany<super::artist::Entity>,
    #[sea_orm(has_many)]
    pub tracks: HasMany<super::track::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

impl Serialize for ModelEx {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Album", 8)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("plays", &self.plays)?;
        state.serialize_field("lastPlayed", &self.last_played)?;
        state.serialize_field("lastModified", &self.last_modified)?;
        state.serialize_field("musicbrainzId", &self.musicbrainz_id)?;
        state.serialize_field(
            "artists",
            &self
                .artists
                .iter()
                .map(|a| a.name.clone())
                .collect::<Vec<_>>(),
        )?;
        state.serialize_field("tracks", &self.tracks)?;
        state.end()
    }
}

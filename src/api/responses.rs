use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::db::{album, track};

const HARMONY_VERSION: &str = "0.1.0";
const SERVER_TYPE: &str = "harmony";
const SERVER_VERSION: &str = "0.1.0";

pub struct HarmonyResponse {
    pub status: Result<(), String>,
    pub with_license: bool,
}

impl Serialize for HarmonyResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // serialize default values
        let mut state = serializer.serialize_struct("HarmonyResponse", 6)?;
        state.serialize_field(
            "status",
            match &self.status {
                Ok(_) => "ok",
                _ => "failed",
            },
        )?;
        state.serialize_field("version", HARMONY_VERSION)?;
        state.serialize_field("type", SERVER_TYPE)?;
        state.serialize_field("serverVersion", SERVER_VERSION)?;

        // if the status is an error, add an error field to the response and stop
        if let Err(e) = &self.status {
            state.serialize_field("error", e)?;
            return state.end();
        }

        // if with_license is true, add an always valid license
        if self.with_license {
            state.serialize_field("license", &HarmonyLicense {})?;
        }

        return state.end();
    }
}

struct HarmonyLicense {}

impl Serialize for HarmonyLicense {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // return an always valid license
        let mut state = serializer.serialize_struct("HarmonyLicense", 1)?;
        state.serialize_field("valid", &true)?;
        state.end()
    }
}

#[derive(serde::Serialize)]
pub struct AlbumListResponse {
    pub harmony: HarmonyResponse,
    pub albums: Vec<album::ModelEx>,
}

#[derive(serde::Serialize)]
pub struct AlbumResponse {
    pub harmony: HarmonyResponse,
    pub album: Option<album::ModelEx>,
}

#[derive(serde::Serialize)]
pub struct TrackResponse {
    pub harmony: HarmonyResponse,
    pub track: Option<track::ModelEx>,
}

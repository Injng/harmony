use anyhow::Result;

pub trait Track {
    // required metadata fields
    fn get_album_name(&self) -> Result<String>;
    fn get_track_name(&self) -> Result<String>;
    fn get_artists(&self) -> Result<Vec<String>>;

    // optional metadata fields
    fn get_album_artists(&self) -> Option<Vec<String>>;
    fn get_musicbrainz_album_id(&self) -> Option<String>;
}

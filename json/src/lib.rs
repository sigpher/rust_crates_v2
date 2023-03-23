use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Photo<'a> {
    #[serde(rename = "albumId")]
    pub album_id: u8,
    pub id: u8,
    pub title: &'a str,
    pub url: &'a str,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: &'a str,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    #[serde(rename = "albumId")]
    pub album_id: u8,
    pub id: u8,
    pub title: String,
    pub url: String,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct News {
//     #[serde(rename = "sourceId")]
//     pub source_id: String,
//     #[serde(rename = "riskLevel")]
//     pub risk_level: i64,
//     #[serde(rename = "upTimes")]
//     pub up_times: i64,
//     pub lmodify: String,
//     pub source: String,
//     pub postid: String,
//     pub title: String,
//     pub mtime: String,
//     pub topicid: String,
//     pub digest: String,
//     pub boardid: String,
//     pub imgsrc: String,
//     pub ptime: String,
//     pub daynum: String,
//     #[serde(rename = "extraShowFields")]
//     pub extra_show_fields: String,
//     pub votecount: i64,
//     pub docid: String,
//     pub url_3w: String,
//     pub priority: i64,
//     #[serde(rename = "downTimes")]
//     pub down_times: i64,
//     pub url: String,
//     pub quality: i64,
//     #[serde(rename = "commentStatus")]
//     pub comment_status: i64,
//     #[serde(rename = "replyCount")]
//     pub reply_count: i64,
//     pub ltitle: String,
//     pub subtitle: String,
// }

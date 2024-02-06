use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StreamJson {
    pub _id: String,
    pub name: Option<String>,
    pub media: String,
    pub provider: String,
    pub date_added: String,
    pub ident: String,
    pub size: i64,
    pub audio: Vec<Audio>,
    pub video: Vec<Video>,
    pub subtitles: Vec<Subtitle>,

}

#[derive(Serialize, Deserialize)]
pub struct Audio {
    pub language: String,
    pub codec: String,
    pub channels: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Video {
    pub width: i32,
    pub height: i32,
    pub codec: String,
    pub aspect: f32,
    // pub 3d: bool,
    pub duration: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Subtitle {
    pub _id: Option<String>,
    pub language: String,
    pub forced: bool,
    pub src: Option<String>,
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchJson {
    pub took: i32,
    pub timed_out: bool,
    pub _shards: Shards,
    pub hits: Hits,
}

#[derive(Serialize, Deserialize)]
pub struct Shards {
    pub total: i32,
    pub successful: i32,
    pub failed: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Hits {
    pub total: HitsTotal,
    pub max_score: Option<f32>,
    pub hits: Vec<Hit>,
}

#[derive(Serialize, Deserialize)]
pub struct HitsTotal {
    pub value: i32,
    pub relation: String,
}

#[derive(Serialize, Deserialize)]
pub struct Hit {
    pub _index: String,
    pub _id: String,
    pub _score: f32,
    pub _ignored: Option<Vec<String>>,
    pub _source: Source,
}

#[derive(Serialize, Deserialize)]
pub struct Ratings {
    pub tmdb: Option<Rating>,
    pub trakt: Option<Rating>,
    pub overall: Option<Rating>
}
#[derive(Serialize, Deserialize)]
pub struct Rating {
    pub rating: f32,
    pub votes: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    pub original_language: Option<String>,
    pub languages: Vec<String>,
    pub ratings: Ratings,
    pub videos: Vec<Video>,
    pub revenue: Option<i64>,
    pub budget: Option<i64>,
    pub info_labels: InfoLabels,
    pub cast: Vec<Cast>,
    pub i18n_info_labels: Vec<I18nInfoLabels>,
    pub services: Services,
    pub available_streams: AvailableStreams,
    pub stream_info: StreamInfo,
    pub popularity: f32,
    pub trending: f32,
    pub play_count: i32,
    pub child_count: Option<i32>,
    pub total_children_count: Option<i32>,
    pub is_concert: Option<bool>,
    pub tags: Vec<String>,
    pub streams_format_into: Option<StreamsFormatInto>,
}

#[derive(Serialize, Deserialize)]
pub struct StreamsFormatInto {
    pub hdr: bool,
    pub dv: bool,
    // pub 3d: bool,
    pub hdr_formats: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StreamVideo {
    pub codec: String,
    pub width: i32,
    pub height: i32,
    pub aspect: f32,
    pub duration: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct StreamAudio {
    pub codec: String,
    pub channels: i32,
    pub language: String,
}

#[derive(Serialize, Deserialize)]
pub struct Subtitles {
    pub language: String,
}

#[derive(Serialize, Deserialize)]
pub struct StreamInfo {
    pub video: Option<StreamVideo>,
    pub subtitles: Option<Subtitles>,
    pub audio: Option<StreamAudio>,
}

#[derive(Serialize, Deserialize)]
pub struct LanguagesAudio {
    pub items:Vec<LanguagesAudioItem>,
    pub map: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct LanguagesAudioItem {
    pub lang: String,
    pub date_added: String
}
#[derive(Serialize, Deserialize)]
pub struct LanguagesSubtitles {
    pub items: Vec<LanguagesSubtitlesItem>,
    pub map: Vec<String>
}
#[derive(Serialize, Deserialize)]
pub struct LanguagesSubtitlesItem {
    pub lang: String,
    pub date_added: String
}
#[derive(Serialize, Deserialize)]
pub struct Languages {
    pub audio: LanguagesAudio,
    pub subtitles: LanguagesSubtitles,
}
#[derive(Serialize, Deserialize)]
pub struct AvailableStreams {
    pub languages: Languages,
    pub count: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Services {
    pub csfd: Option<String>,
    pub imdb: Option<String>,
    pub trakt: Option<String>,
    pub trakt_with_type: Option<String>,
    pub slug: Option<String>,
    pub tmdb: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct I18nInfoLabels {
    pub lang: String,
    pub title: Option<String>,
    pub plot: Option<String>,
    pub tagline: Option<String>,
    pub art: Option<Art>,
}
#[derive(Serialize, Deserialize)]
pub struct Art {
    pub poster: Option<String>,
    pub fanart: Option<String>,
    pub thumb: Option<String>,
    pub banner: Option<String>,
    pub clearart: Option<String>,
    pub clearlogo: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Cast {
    pub name: String,
    pub role: Option<String>,
    pub thumbnail: Option<String>,
    pub order: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct InfoLabels {
    pub originaltitle: Option<String>,
    pub genre: Vec<String>,
    pub year: i32,
    pub director: Vec<String>,
    pub mpaa: Option<String>,
    pub studio: Vec<String>,
    pub writer: Vec<String>,
    pub premiered: Option<String>,
    pub dateadded: String,
    pub mediatype: String,
    pub country: Vec<Option<String>>,
    pub status: Option<String>,
    pub duration: Option<i32>,
}
#[derive(Serialize, Deserialize)]
pub struct Subtitle {
    pub src: Option<String>,
    pub lang: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Video {
    pub name: Option<String>,
    pub size: Option<i32>,
    pub r#type: String,
    pub url: String,
    pub lang: Option<String>,
    pub subtitles: Option<Vec<Subtitle>>,
}

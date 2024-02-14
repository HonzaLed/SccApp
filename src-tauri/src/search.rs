use crate::{SearchJson, StreamJson};
use reqwest;
use serde_json;

static API: &str = "http://plugin.sc2.zone/api";
static TOKEN: &str = "&access_token=9ajdu4xyn1ig8nxsodr3";

pub fn search(search_string: &str, size: i32) -> SearchJson {
    let url = format!("{}/media/filter/v2/search?{}&order=desc&size={size}&sort=score&type=%2A&value={}", API, TOKEN, search_string);
    let resp: String = reqwest::blocking::get(url).unwrap().text().unwrap();
    // fs::write("search.json", &resp).expect("Unable to write file");
    parse_search(resp)
}

fn parse_search(text: String) -> SearchJson {
    let search_json: Result<SearchJson, serde_json::Error> = serde_json::from_str(&text);
    if let Err(error) = &search_json {
        panic!("Error parsing the JSON data: {}", error);
    }
    search_json.unwrap()
}

pub fn get_streams(id: &str) -> Result<Vec<StreamJson>, String> {
    let url = format!("{}/media/{}/streams?{}", API, id, TOKEN);
    // http://plugin.sc2.zone/api/media/5ed13ce408570680401c96be/streams?access_token=9ajdu4xyn1ig8nxsodr3
    let resp: String = reqwest::blocking::get(url).unwrap().text().unwrap();
    // fs::write("streams.json", &resp).expect("Unable to write file");
    parse_streams(resp)
}

fn parse_streams(text: String) -> Result<Vec<StreamJson>, String> {
    let stream_json: Result<Vec<StreamJson>, serde_json::Error> = serde_json::from_str(&text);
    if let Err(error) = &stream_json {
        return Err(error.to_string());
    }
    Ok(stream_json.unwrap())
}
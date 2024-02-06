// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use bytesize::{self, ByteSize};

use search_def::SearchJson;
mod search_def;
mod search;

mod stream_def;
use stream_def::StreamJson;

mod webshare_def;
mod webshare;

mod config_def;
mod config;


use serde_json;

use std::fs::{self, File};

#[derive(serde::Serialize)]
struct LoginResponse {
    success: bool,
    token: String,
    error: String,
}
#[derive(serde::Serialize)]
struct FileLinkResponse {
    success: bool,
    link: String,
    error: String,
}
#[derive(serde::Serialize)]
struct CredsResponse {
    success: bool,
    creds: (String, String),
    error: String,
}
#[derive(serde::Serialize)]
struct SetCredsResponse {
    success: bool,
    error: String,
}

#[tauri::command]
fn cmd_get_creds() -> CredsResponse {
    let config = config::load_config("config.toml");
    let creds = config::load_creds(config);
    CredsResponse {
        success: true,
        creds: creds,
        error: String::from(""),
    }
}

#[tauri::command]
fn cmd_save_creds(username: &str, password: &str) -> SetCredsResponse {
    let result = config::save_creds("config.toml", username, password);
    if result.is_err() {
        return SetCredsResponse {
            success: false,
            error: result.err().unwrap(),
        }
    } else {
        return SetCredsResponse {
            success: true,
            error: String::from(""),
        }
    }
}

#[tauri::command]
fn cmd_login(username: &str, password: &str) -> LoginResponse {
    let token = webshare::login(username.to_string(), password.to_string());
    if token.is_err() {
        return LoginResponse {
            success: false,
            token: String::from(""),
            error: token.err().unwrap(),
        }
    } else {
        return LoginResponse {
            success: true,
            token: token.unwrap(),
            error: String::from(""),
        }
    }
}

#[tauri::command]
fn cmd_search(query: String) -> SearchJson {
    search::search(&query, 10)
}

// #[derive(Default)]
// struct MyState {
//   s: std::sync::Mutex<String>,
//   t: std::sync::Mutex<std::collections::HashMap<String, String>>,
// }
// // remember to call `.manage(MyState::default())`
#[tauri::command]
fn cmd_get_streams(id: &str) -> Vec<StreamJson> {
    search::get_streams(id)
}

#[tauri::command]
fn cmd_get_file_link(name: &str, ident: &str, token: &str) -> FileLinkResponse {
    let result = webshare::get_file_link(name, ident, &token.to_string());
    if result.is_err() {
        return FileLinkResponse {
            success: false,
            link: String::from(""),
            error: result.err().unwrap(),
        }
    } else {
        return FileLinkResponse {
            success: true,
            link: result.unwrap(),
            error: String::from(""),
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cmd_login, cmd_search, cmd_get_file_link, cmd_get_streams, cmd_get_creds, cmd_save_creds])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

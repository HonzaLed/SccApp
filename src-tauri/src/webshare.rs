use reqwest;
use std::str;

use pwhash::{md5_crypt, HashSetup};

use serde_xml_rs;

use crate::{stream_def::StreamJson, webshare_def::{SaltXML, LoginXML, FileLinkXML}};

use sha1::{Sha1, Digest};
// use sha256::Sha256;

// use std::fs;

static WEBSHARE_API: &str = "https://webshare.cz/api";

fn get_salt(username: String) -> Result<String, String> {
    let url = format!("{}/salt/", WEBSHARE_API);
    let params: &[(&str, &str); 1] = &[("username_or_email", &username)]; //format!("username_or_email={}&wst=", username);
    let client = reqwest::blocking::Client::new();
    let resp = client.post(url).form(params).send().unwrap().text().unwrap();

    // fs::write("salt.txt", &resp).expect("Unable to write file");
    parse_salt_xml(resp)
}

fn parse_salt_xml(resp: String) -> Result<String, String> {
    let salt_xml = serde_xml_rs::from_str(&resp);
    if salt_xml.is_err() {
        let err = format!("Error parsing salt: {}", salt_xml.err().unwrap());
        println!("{}", err);
        return Err(err);
    }
    let salt_xml: SaltXML = salt_xml.unwrap();
    if salt_xml.status.to_lowercase() != "ok" {
        let err = format!("Error getting salt: {}", salt_xml.status);
        println!("{}", err);
        return Err(err);
    }
    if salt_xml.salt.is_none() {
        println!("Error getting salt: salt is None");
        return Err(String::from("Error getting salt: salt is None"));
    }
    Ok(salt_xml.salt.unwrap())
}

pub fn login(username: String, password: String) -> Result<String, String> {
    let salt = get_salt(username.clone());
    if salt.is_err() {
        let err = format!("Error getting salt: {}", salt.err().unwrap());
        println!("{}", err);
        return Err(err);
    }
    let salt = salt.unwrap();
    let hash = webshare_hash(password, &salt);
    if hash == "" {
        println!("Error generating hash!");
        return Err(String::from("Error generating hash!"));
    }

    let url = format!("{}/login/", WEBSHARE_API);
    let params = &[("username_or_email", &username), ("password", &hash), ("keep_logged_in", &"0".to_string())];

    let client = reqwest::blocking::Client::new();
    let resp = client.post(url).form(params).send().unwrap().text().unwrap();

    parse_login_xml(resp)
}

fn parse_login_xml(resp: String) -> Result<String, String> {
    let login_xml = serde_xml_rs::from_str(&resp);
    if login_xml.is_err() {
        let err = format!("Error parsing login: {}", login_xml.err().unwrap());
        println!("{}", err);
        return Err(err);
    }
    let login_xml: LoginXML = login_xml.unwrap();
    if login_xml.status.to_lowercase() != "ok" {
        let err = format!("Error getting login: {}", login_xml.code.unwrap());
        println!("{}", err);
        return Err(err);
    }
    if login_xml.token.is_none() {
        println!("Error getting login: token is None");
        return Err(String::from("Error getting login: token is None"));
    }
    Ok(login_xml.token.unwrap())
}

fn webshare_hash(password: String, salt: &str) -> String {
    let hash_setup = HashSetup{ salt: Some(salt), rounds: Some(1000) };
    #[allow(deprecated)]
    let md5_hash = md5_crypt::hash_with(hash_setup, password);
    if md5_hash.is_err() {
        println!("Error generating md5crypt: {}", md5_hash.err().unwrap());
        return String::from("");
    }
    let md5_hash = md5_hash.unwrap();

    let sha1_hash = Sha1::digest(md5_hash.as_bytes());
    
    sha1_hash.iter().map(|byte| format!("{:02x}", byte)).collect()
}

pub fn get_file_link(name: &str, ident: &str, token: &String) -> Result<String, String> {
    let file_salt = get_file_salt(ident);
    let mut password: String = String::new();
    if file_salt.is_err() {
        let err = format!("Error getting file_salt: {}", file_salt.err().unwrap());
        println!("{}", err);
        return Err(String::from(err));
    }
    let file_salt = file_salt.unwrap();
    if file_salt.is_some() {
        let file_salt = file_salt.unwrap();
        let file_password = get_file_password(name, ident, &file_salt);
        if file_password.is_err() {
            let err = format!("Error getting file_password: {}", file_password.err().unwrap());
            println!("{}", err);
            return Err(String::from(err));
        }
        password = file_password.unwrap();
    }
    let url = format!("{}/file_link/", WEBSHARE_API);
    let params = &[("download_type", &String::from("video_stream")), ("ident", &ident.to_string()), ("password", &password), ("wst", token)];
    let resp = reqwest::blocking::Client::new().post(url).form(params).send().unwrap().text().unwrap();
    parse_file_link_xml(resp)
}

fn parse_file_link_xml(resp: String) -> Result<String, String> {
    let file_link_xml = serde_xml_rs::from_str(&resp);
    if file_link_xml.is_err() {
        let err = format!("Error parsing file_link: {}", file_link_xml.err().unwrap());
        println!("{}", err);
        return Err(String::from(err));
    }
    let file_link_xml: FileLinkXML = file_link_xml.unwrap();
    if file_link_xml.status.to_lowercase() != "ok" {
        let err = format!("Error getting file_link: {}", file_link_xml.code.unwrap());
        println!("{}", err);
        return Err(String::from(err));
    }
    if file_link_xml.link.is_none() {
        println!("Error getting file_link: file_link is None");
        return Err(String::from("Error getting file_link: file_link is None"));
    }
    Ok(file_link_xml.link.unwrap())
}

fn get_file_password(name: &str, ident: &str, salt: &str) -> Result<String, String> {
    use sha2::Sha256;

    let password = format!("{}{}", name, ident);
    let password = Sha256::digest(password.as_bytes());
    let password = password.iter().map(|byte| format!("{:02x}", byte)).collect();
    Ok(webshare_hash(password, salt))
}

fn get_file_salt(ident: &str) -> Result<Option<String>, String> {
    let url = format!("{}/file_password_salt/", WEBSHARE_API);
    let params = &[("ident", ident)];
    let resp = reqwest::blocking::Client::new().post(url).form(params).send().unwrap().text().unwrap();
    parse_file_salt_xml(resp)
}

fn parse_file_salt_xml(resp: String) -> Result<Option<String>, String> {
    let salt_xml = serde_xml_rs::from_str(&resp);
    if salt_xml.is_err() {
        println!("Error parsing file_salt: {}", salt_xml.err().unwrap());
        return Err(String::from("Error parsing XML!"));
    }
    let salt_xml: SaltXML = salt_xml.unwrap();
    if salt_xml.status.to_lowercase() != "ok" {
        let code = salt_xml.code.unwrap();
        if code == "FILE_PASSWORD_SALT_FATAL_2" {
            return Ok(None);
        }
        return Err(code);
    }
    if salt_xml.salt.is_none() {
        println!("Error getting file_salt: salt is None");
        return Err(String::from("Salt is none!"));
    }
    Ok(Some(salt_xml.salt.unwrap()))
}
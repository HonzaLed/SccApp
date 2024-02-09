use reqwest;
use std::str;

use pwhash::{md5_crypt, HashSetup};

use serde_xml_rs;

use crate::webshare_def::{SaltXML, LoginXML, FileLinkXML};

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
    
    if let Err(e) = salt_xml {
        return Err(e.to_string());
    }
    
    let salt_xml: SaltXML = salt_xml.unwrap();
    if salt_xml.status.to_lowercase() != "ok" {
        return Err(salt_xml.status);
    }
    if salt_xml.salt.is_none() {
        return Err(String::from("Salt is None"));
    }
    Ok(salt_xml.salt.unwrap())
}

pub fn login(username: String, password: String) -> Result<String, String> {
    let salt = get_salt(username.clone());
    
    if let Err(e) = salt {
        return Err(e);
    }
    
    let salt = salt.unwrap();
    let hash = webshare_hash(password, &salt);
    
    if hash.is_err() {
        return hash;
    }
    let hash = hash.unwrap();

    let url = format!("{}/login/", WEBSHARE_API);
    let params = &[("username_or_email", &username), ("password", &hash), ("keep_logged_in", &"0".to_string())];

    let client = reqwest::blocking::Client::new();
    let resp = client.post(url).form(params).send().unwrap().text().unwrap();

    parse_login_xml(resp)
}

fn parse_login_xml(resp: String) -> Result<String, String> {
    let login_xml = serde_xml_rs::from_str(&resp);
    
    if let Err(e) = login_xml {
        return Err(e.to_string());
    }
    
    let login_xml: LoginXML = login_xml.unwrap();
    if login_xml.status.to_lowercase() != "ok" {
        return Err(login_xml.code.unwrap());
    }
    if login_xml.token.is_none() {
        return Err(String::from("token is None"));
    }
    Ok(login_xml.token.unwrap())
}

fn webshare_hash(password: String, salt: &str) -> Result<String, String> {
    let hash_setup = HashSetup{ salt: Some(salt), rounds: Some(1000) };
    #[allow(deprecated)]
    let md5_hash = md5_crypt::hash_with(hash_setup, password);
    
    if let Err(e) = md5_hash {
        println!("Error generating md5crypt: {}", e);
        return Err(e.to_string());
    }
    
    let md5_hash = md5_hash.unwrap();

    let sha1_hash = Sha1::digest(md5_hash.as_bytes());
    
    Ok(sha1_hash.iter().map(|byte| format!("{:02x}", byte)).collect())
}

pub fn get_file_link(name: &str, ident: &str, token: &String) -> Result<String, String> {
    let file_salt = get_file_salt(ident);
    let mut password: String = String::new();
    
    if let Err(e) = file_salt {
        return Err(e);
    }

    let file_salt = file_salt.unwrap();
    if file_salt.is_some() {
        let file_salt = file_salt.unwrap();
        let file_password = get_file_password(name, ident, &file_salt);
        
        if let Err(e) = file_password {
            return Err(e);
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
    
    if let Err(e) = file_link_xml {
        return Err(e.to_string());
    }
    
    let file_link_xml: FileLinkXML = file_link_xml.unwrap();
    if file_link_xml.status.to_lowercase() != "ok" {
        return Err(file_link_xml.code.unwrap());
    }
    if file_link_xml.link.is_none() {
        println!("Error getting file_link: file_link is None");
        return Err(String::from("file_link is None"));
    }
    Ok(file_link_xml.link.unwrap())
}

fn get_file_password(name: &str, ident: &str, salt: &str) -> Result<String, String> {
    use sha2::Sha256;

    let password = format!("{}{}", name, ident);
    let password = Sha256::digest(password.as_bytes());
    let password = password.iter().map(|byte| format!("{:02x}", byte)).collect();
    webshare_hash(password, salt)
}

fn get_file_salt(ident: &str) -> Result<Option<String>, String> {
    let url = format!("{}/file_password_salt/", WEBSHARE_API);
    let params = &[("ident", ident)];
    let resp = reqwest::blocking::Client::new().post(url).form(params).send().unwrap().text().unwrap();
    parse_file_salt_xml(resp)
}

fn parse_file_salt_xml(resp: String) -> Result<Option<String>, String> {
    let salt_xml = serde_xml_rs::from_str(&resp);

    if let Err(e) = salt_xml {
        println!("Error parsing file_salt: {}", e);
        return Err(e.to_string());
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
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use log::{debug, error, info};
use serde::{Deserialize, Serialize};

extern crate xdg;

extern crate reqwest;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

fn find_identity_file() -> Option<PathBuf> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("mycroft/identity").unwrap();
    xdg_dirs.find_config_files("identity2.json").next()
}

fn current_epoch() -> f32 { 
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default().as_secs_f32()
}


#[allow(dead_code)]
pub fn get_identity() -> Option<Identity> {
    let mut result = None;
    if let Some(identity_file) = find_identity_file() {
        info!("identity file: {:?}", identity_file);
        if let Ok(json_string) = fs::read_to_string(identity_file) {
            let json_str = json_string.as_str();
            debug!("Content: {}", json_str);
            
            match serde_json::from_str(json_str) {
                Ok(identity) => {
                    let parsed: Identity = identity;
                    result = Some(parsed);
                },
                Err(e) => {error!("{}", e);}
            };
        }
    } else {
        error!("identity2.json was not found");
    }
    result 
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdentityServerResponse {
    pub uuid: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expiration: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
    pub uuid: String,
    pub access: String,
    pub refresh: String,
    pub expires_at: f32
}

impl Identity {
    #[allow(dead_code)]
    pub fn load() -> Option<Identity> {
        get_identity()
    }

    pub fn is_expired(&self) -> bool {
        current_epoch() > self.expires_at
    }

    pub fn refresh(&mut self) {
        let refresh_token = &self.refresh;
        let client = reqwest::blocking::Client::new();
        let mut headers = HeaderMap::new();
        let mut bearer_auth = String::from("Bearer ");
        bearer_auth.push_str(refresh_token);
        debug!("bearer_auth: {}", bearer_auth);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(bearer_auth.as_str()).unwrap());
        headers.insert("Device", HeaderValue::from_str(self.uuid.as_str()).unwrap());
        let response = client.get("https://api.mycroft.ai/v1/auth/token")
            .headers(headers)
            .send().unwrap();

        match response.json::<IdentityServerResponse>() {
        //match response.text() {
            Ok(identity) => {
                debug!("New identity!");
                debug!("{:?}", identity);
                self.update(identity);
                write_identity_file(self);
            },
            Err(e) => {
                error!("{}", e);
            }
        };
    }

    fn update(&mut self, identity: IdentityServerResponse) {
        self.access = identity.access_token;
        self.refresh = identity.refresh_token;
        self.expires_at = current_epoch() + identity.expiration;
        self.uuid = identity.uuid;
    }
}

fn write_identity_file(identity: &Identity) {
    if let Some(identity_file) = find_identity_file() {
        let json_data = serde_json::to_string(&identity).unwrap();
        fs::write(identity_file, json_data).expect("Unable to write file");
    } else {
        error!("Couldn't find identity file location");
    }
}

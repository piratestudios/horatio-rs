// Crates

extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

// Imports

use chrono::prelude::{DateTime, Utc};
use reqwest::{Client, Error, StatusCode};

// Types

/// A NewPing is a value that gets sent to Horatio.
#[derive(Deserialize, Debug, PartialEq, Serialize)]
pub struct NewPing {
    pub id: String, // uuid that corresponds to the device id
    #[serde(rename = "type")]
    pub type_: String, // type of ping e.g. "resin"
}

/// A Ping is a value that Horatio uses to indicate when a device was last seen.
#[derive(Deserialize, Debug, PartialEq, Serialize)]
pub struct Ping {
    pub id: String, // uuid that corresponds to the device id
    pub last_seen: DateTime<Utc>, // time when the device last phoned home
    #[serde(rename = "type")]
    pub type_: String, // type of ping e.g. "resin"
}

#[derive(Debug)]
pub enum ApiError {
    Reqwest(Error),
    Server(StatusCode),
}

// Functions

pub fn ping(base_url: &str, id: &String) -> Result<Ping, ApiError> {
    // Create path.
    let path = format!("{}/ping", base_url);

    // Instantiate new client.
    let client = Client::new();

    // Format JSON.
    let ping = NewPing {
        id: id.clone(),
        type_: format!("resin"),
    };
    let body = serde_json::to_string(&ping).unwrap();

    // Call Horatio.
    match client.post(&path).body(body).send() {
        Err(err) => Err(ApiError::Reqwest(err)),

        Ok(mut response) => {
            if response.status().is_success() {
                match response.json() {
                    Err(err) => Err(ApiError::Reqwest(err)),
                    Ok(result) => Ok(result),
                }
            } else {
                Err(ApiError::Server(response.status()))
            }
        }
    }
}

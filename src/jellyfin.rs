use std::collections::HashMap;

use reqwest::{self, Error};
use serde::Deserialize;

mod credentials;

// JellyFin requires X-Emby-Authorisation
// more info here https://github.com/MediaBrowser/Emby/wiki/User-Authentication
// TODO: get actual device ID
const AUTH_HEADER: &str = 
  "MediaBrowser Client=\"Jellyfin Web\", \
  DeviceId=\"0146dd77-ef5f-49c3-818c-f6949909305e\", \
  Version=\"0.1.0\"";

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct JellyFinLoginUserRes {
  id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct JellyFinLoginRes {
  user: JellyFinLoginUserRes,
  access_token: String,
}

pub struct User {
  id: String,
  token: String,
}

fn login() -> Result<User, Error> {
  let mut login_credentials = HashMap::new();
  login_credentials.insert("Pw", credentials::JELLYFIN_PASSWORD);
  login_credentials.insert("Username", credentials::JELLYFIN_USERNAME);

  let client = reqwest::blocking::Client::new();
  let res = client
    .post(credentials::JELLYFIN_URL.to_owned() + "/Users/AuthenticateByName")
    .header("X-Emby-Authorization", AUTH_HEADER)
    .json(&login_credentials)
    .send()?;
  
  let body = res.json::<JellyFinLoginRes>()?;

  let user = User {
    id: body.user.id,
    token: body.access_token,
  };

  return Ok(user);
}

pub fn is_film_on_jellyfin() -> bool {
  let user = match login(){
    Ok(user) => user,
    Err(error) => {
      eprintln!("Failed to login to JellyFin: {}", error);
      return false;
    }
  };
  println!("{} {}", user.id, user.token);
  return false;
}
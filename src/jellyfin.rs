use std::collections::HashMap;

use reqwest;
use serde::Deserialize;

use crate::film::Film;

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

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct JellyFinSearchItemRes {
  name: String,
  production_year: u16,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct JellyFinSearchRes {
  items: Vec<JellyFinSearchItemRes>,
}

pub struct User {
  id: String,
  token: String,
}

fn login() -> Result<User, String> {
  let mut login_credentials = HashMap::new();
  login_credentials.insert("Pw", credentials::JELLYFIN_PASSWORD);
  login_credentials.insert("Username", credentials::JELLYFIN_USERNAME);

  let client = reqwest::blocking::Client::new();
  let res = match client
    .post(credentials::JELLYFIN_URL.to_owned() + "/Users/AuthenticateByName")
    .header("X-Emby-Authorization", AUTH_HEADER)
    .json(&login_credentials)
    .send() {
      Ok(res) => res,
      Err(error) => {
        let err_msg = format!("Failed to login: {}", error);
        eprintln!("{err_msg}");
        return Err(err_msg);
      }
    };
  
  let body = match res.json::<JellyFinLoginRes>() {
    Ok(json) => json,
    Err(error) => {
      let err_msg = format!("Failed to parse response body: {}", error);
      eprintln!("{err_msg}");
      return Err(err_msg);
    }
  };

  let user = User {
    id: body.user.id,
    token: body.access_token,
  };

  return Ok(user);
}

fn search(film: &Film, user: &User) -> Result<bool, String> { 
  let url_path = credentials::JELLYFIN_URL.to_owned() + "/Users/" + &user.id + "/Items";
  let url = match reqwest::Url::parse_with_params(&url_path,
    &[ ("IncludeItemTypes", "Movie")
            ,("Limit", "1")
            ,("Recursive", "true")
            ,("searchTerm", &film.title)
            ,("years", &film.year)
          ]
  ) {
    Ok(url) => url,
    Err(error) => {
      let err_msg = format!("Failed to parse URL: {}", error);
      eprintln!("{err_msg}");
      return Err(err_msg);
    },
  };

  let client = reqwest::blocking::Client::new();
  let res = match client
    .get(url)
    .header("X-Emby-Authorization", AUTH_HEADER.to_owned() + ", Token=\"" + &user.token + "\"")
    .send() {
      Ok(res) => res,
      Err(error) => {
        let err_msg = format!("Failed to search: {}", error);
        eprintln!("{err_msg}");
        return Err(err_msg);
      }
    };

  let body = match res.json::<JellyFinSearchRes>() {
    Ok(json) => json,
    Err(error) => {
      let err_msg = format!("Failed to parse response body: {}", error);
      eprintln!("{err_msg}");
      return Err(err_msg);
    }
  };

  return Ok(!body.items.is_empty());
}

pub fn is_film_on_jellyfin(film: &Film) -> bool {
  let user = match login(){
    Ok(user) => user,
    Err(error) => {
      eprintln!("Failed to login to JellyFin: {}", error);
      return false;
    }
  };
  
  return match search(film, &user) {
    Ok(found) => found,
    Err(_error) => {
      return false;
    }
  }
}
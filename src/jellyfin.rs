use std::collections::HashMap;

use reqwest;

use crate::film::Film;

mod api;
mod credentials;

pub struct User {
  id: String,
  token: String,
}

fn get_auth_header(user_token: Option<&String>) -> String {
  let version = env!("CARGO_PKG_VERSION");
  // JellyFin requires X-Emby-Authorisation
  // more info here https://github.com/MediaBrowser/Emby/wiki/User-Authentication
  // TODO: get actual device ID
  let auth_header =  
    "MediaBrowser Client=\"Jellyfin Web\", \
    DeviceId=\"0146dd77-ef5f-49c3-818c-f6949909305e\", \
    Version=\"".to_owned() + version + "\"";

  return match user_token {
    Some(token) => auth_header + ", Token=\"" + &token + "\"",
    None => auth_header, 
  };
}

fn search(client: &reqwest::blocking::Client, film: &Film, user: &User) -> Result<bool, String> { 
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

  let res = match client
    .get(url)
    .header("X-Emby-Authorization", get_auth_header(Some(&user.token)))
    .send() {
      Ok(res) => res,
      Err(error) => {
        let err_msg = format!("Failed to search: {}", error);
        eprintln!("{err_msg}");
        return Err(err_msg);
      }
    };

  let body = match res.json::<api::JellyFinSearchRes>() {
    Ok(json) => json,
    Err(error) => {
      let err_msg = format!("Failed to parse response body: {}", error);
      eprintln!("{err_msg}");
      return Err(err_msg);
    }
  };

  return Ok(!body.items.is_empty());
}

pub fn init() -> reqwest::blocking::Client {
  return reqwest::blocking::Client::new();
}

pub fn login(client: &reqwest::blocking::Client) -> Result<User, String> {
  let mut login_credentials = HashMap::new();
  login_credentials.insert("Pw", credentials::JELLYFIN_PASSWORD);
  login_credentials.insert("Username", credentials::JELLYFIN_USERNAME);

  let res = match client
    .post(credentials::JELLYFIN_URL.to_owned() + "/Users/AuthenticateByName")
    .header("X-Emby-Authorization", get_auth_header(None))
    .json(&login_credentials)
    .send() {
      Ok(res) => res,
      Err(error) => {
        let err_msg = format!("Failed to login: {}", error);
        eprintln!("{err_msg}");
        return Err(err_msg);
      }
    };
  
  let body = match res.json::<api::JellyFinLoginRes>() {
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

pub fn is_film_on_jellyfin(client: &reqwest::blocking::Client, film: &Film, user: &User) -> bool {  
  return match search(client, film, user) {
    Ok(found) => found,
    Err(_error) => {
      return false;
    }
  }
}
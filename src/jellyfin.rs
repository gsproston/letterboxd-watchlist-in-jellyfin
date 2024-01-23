use std::collections::{HashMap, HashSet};

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
    let auth_header = "MediaBrowser Client=\"Jellyfin Web\", \
    DeviceId=\"0146dd77-ef5f-49c3-818c-f6949909305e\", \
    Version=\""
        .to_owned()
        + version
        + "\"";

    return match user_token {
        Some(token) => auth_header + ", Token=\"" + &token + "\"",
        None => auth_header,
    };
}

pub fn get_all_films(
    client: &reqwest::blocking::Client,
    user: &User,
    years: HashSet<String>,
) -> Result<Vec<Film>, String> {
    let years_filter = years
        .iter()
        .map(|year| year.to_owned() + ",")
        .collect::<String>();
    let url_path = credentials::JELLYFIN_URL.to_owned() + "/Users/" + &user.id + "/Items";
    let url = match reqwest::Url::parse_with_params(
        &url_path,
        &[
            ("IncludeItemTypes", "Movie"),
            ("Recursive", "true"),
            ("years", &years_filter),
        ],
    ) {
        Ok(url) => url,
        Err(error) => {
            let err_msg = format!("Failed to parse URL: {}", error);
            eprintln!("{err_msg}");
            return Err(err_msg);
        }
    };

    let res = match client
        .get(url)
        .header("X-Emby-Authorization", get_auth_header(Some(&user.token)))
        .send()
    {
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

    let films = body
        .items
        .iter()
        .map(|item| Film {
            title: item.name.to_owned(),
            year: item.production_year.to_string(),
        })
        .collect::<Vec<Film>>();

    return Ok(films);
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
        .send()
    {
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

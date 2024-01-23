use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JellyFinLoginUserRes {
  pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JellyFinLoginRes {
  pub user: JellyFinLoginUserRes,
  pub access_token: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JellyFinSearchItemRes {
  pub name: String,
  pub production_year: u16,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JellyFinSearchRes {
  pub items: Vec<JellyFinSearchItemRes>,
}
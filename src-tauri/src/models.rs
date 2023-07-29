use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Repo {
    pub id: i32,
    pub local_path: String,
    pub url: String,
    pub image_path: Option<String>,
    pub colour: Option<String>,
    pub abb: Option<String>,
}

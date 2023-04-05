use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub base_url: String,
    pub posts: Vec<Post>,
}

#[derive(Deserialize, Debug)]
pub struct Post {
    pub title: String,
    #[serde(default)]
    pub draft: bool,
}

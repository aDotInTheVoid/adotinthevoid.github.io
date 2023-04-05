use camino::Utf8PathBuf;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub base_url: String,
    pub posts: Vec<Post>,
}

#[derive(Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub path: Utf8PathBuf,
    #[serde(default)]
    pub draft: bool,
    pub date: NaiveDate,
}

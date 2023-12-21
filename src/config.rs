use camino::Utf8PathBuf;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub base_url: String,
    pub base_domain: String,
    pub posts: Vec<Post>,

    pub redirects: Vec<Redirect>,
}

#[derive(Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub path: Utf8PathBuf,
    #[serde(default)]
    pub draft: bool,

    #[serde(default)]
    /// Build the post, but don't put on the home page.
    ///
    /// Still put in the RSS feed.
    pub hidden: bool,

    pub date: NaiveDate,
}

#[derive(Deserialize, Debug)]
pub struct Redirect {
    pub from: String,
    pub to: String,
}

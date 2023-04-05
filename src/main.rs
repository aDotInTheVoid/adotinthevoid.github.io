mod config;

use std::io::BufWriter;

use anyhow::Result;
use camino::Utf8Path;
use clap::Parser as _;
use fs_err as fs;
use serde::Serialize;

const BASE_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[derive(clap::Parser)]
struct Args {
    #[clap(long)]
    drafts: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let base_path = Utf8Path::new(BASE_DIR);

    let content = fs::read_to_string(base_path.join("CONTENT.toml"))?;

    let config: config::Config = toml::from_str(&content)?;

    let out_dir = base_path.join("out");
    fs::create_dir_all(&out_dir)?;

    let mut env = minijinja::Environment::new();
    env.add_template("base.html", include_str!("templates/base.html"))?;
    env.add_template("index.html", include_str!("templates/index.html"))?;
    env.add_template("post.html", include_str!("templates/post.html"))?;
    env.add_global("base_url", config.base_url);
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);

    let index_file = fs::File::create(out_dir.join("index.html"))?;
    env.get_template("index.html")?.render_to_write(
        HomeArgs {
            posts: config
                .posts
                .iter()
                .filter(|p| !p.draft || args.drafts)
                .map(|p| HomePostArgs {
                    title: p.title.clone(),
                    url: p.path.with_extension("").to_string(),
                    date: p.date.format("%-d %B %Y").to_string(),
                })
                .collect(),
        },
        BufWriter::new(index_file),
    )?;

    Ok(())
}

#[derive(Serialize)]
struct HomeArgs {
    posts: Vec<HomePostArgs>,
}

#[derive(Serialize)]
struct HomePostArgs {
    title: String,
    url: String,
    date: String,
}

mod config;
mod highlight;
mod markdown;

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

    let posts = config
        .posts
        .iter()
        .filter(|p| !p.draft || args.drafts)
        .map(|p| {
            Ok(HomePostArgs {
                title: p.title.clone(),
                url: p.path.with_extension("").to_string(),
                date: p.date.format("%-d %B %Y").to_string(),
                content: markdown::render(&fs::read_to_string(&p.path)?),
                draft: p.draft,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let index_file = fs::File::create(out_dir.join("index.html"))?;
    env.get_template("index.html")?.render_to_write(
        HomeArgs {
            posts: posts.clone(),
            has_drafts: args.drafts,
        },
        BufWriter::new(index_file),
    )?;

    for p in posts {
        let post_dir = out_dir.join(&p.url);
        fs::create_dir_all(&post_dir)?;
        let post_file = fs::File::create(post_dir.join("index.html"))?;
        env.get_template("post.html")?
            .render_to_write(&p, BufWriter::new(post_file))?;
    }

    Ok(())
}

#[derive(Serialize)]
struct HomeArgs {
    posts: Vec<HomePostArgs>,
    has_drafts: bool,
}

#[derive(Serialize, Clone)]
struct HomePostArgs {
    title: String,
    url: String,
    date: String,
    content: String,
    draft: bool,
}

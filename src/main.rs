mod config;
mod highlight;
mod markdown;
mod rss_channel;

use std::io::BufWriter;

use anyhow::Result;
use camino::Utf8Path;
use clap::Parser as _;
use fs_err as fs;
use rss::validation::Validate;
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
    env.add_global("base_url", config.base_url.clone());
    env.add_global("base_domain", config.base_domain.clone());
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);

    let (posts, items) = config
        .posts
        .iter()
        .filter(|p| !p.draft || args.drafts)
        .map(|p| lower_post(p, &config))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .unzip::<_, _, Vec<_>, Vec<_>>();

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

    let rss_channel = rss_channel::channel(&config, items);
    rss_channel.validate()?;

    fs::write(out_dir.join("index.xml"), rss_channel.to_string())?;

    Ok(())
}

fn lower_post(
    p: &config::Post,
    config: &config::Config,
) -> Result<(HomePostArgs, rss::Item), anyhow::Error> {
    let args = HomePostArgs {
        title: p.title.clone(),
        url: p.path.with_extension("").to_string(),
        date: p.date.format("%-d %B %Y").to_string(),
        content: markdown::render(&config, &fs::read_to_string(&p.path)?),
        draft: p.draft,
    };

    let dt_2822: String = p
        .date
        .and_time(chrono::naive::NaiveTime::MIN)
        .and_local_timezone(chrono::offset::Utc)
        .unwrap()
        .to_rfc2822();

    let link = format!(
        "https://{}{}{}",
        config.base_domain, config.base_url, args.url
    );
    let item = rss::ItemBuilder::default()
        .title(Some(p.title.clone()))
        // .author(Some("Alona Enraght-Moony".to_string()))
        .pub_date(Some(dt_2822))
        .guid(Some(rss::Guid {
            value: link.clone(),
            permalink: true,
        }))
        .link(Some(link))
        .build();

    Ok((args, item))
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

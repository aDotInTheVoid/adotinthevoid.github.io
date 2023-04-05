mod config;

use std::io::BufWriter;

use anyhow::Result;
use camino::Utf8Path;
use fs_err as fs;
use serde::Serialize;

const BASE_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> Result<()> {
    let base_path = Utf8Path::new(BASE_DIR);

    let content = fs::read_to_string(base_path.join("CONTENT.toml"))?;

    let config: config::Config = toml::from_str(&content)?;

    let out_dir = base_path.join("out");
    fs::create_dir_all(&out_dir)?;

    let mut env = minijinja::Environment::new();
    env.add_template("base.html", include_str!("templates/base.html"))?;
    env.add_global("base_url", config.base_url);
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);

    let index_file = fs::File::create(out_dir.join("index.html"))?;

    env.get_template("base.html")?.render_to_write(
        HomeArgs {
            title: "Alona Enraght-Moony",
        },
        BufWriter::new(index_file),
    )?;

    Ok(())
}

#[derive(Serialize)]
struct HomeArgs {
    title: &'static str,
}

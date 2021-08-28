use anyhow::bail;
use std::path::{Path, PathBuf};
use std::env;
use toml_edit::Document;
use std::fs;

fn main() -> anyhow::Result<()> {
    let user_config = PathBuf::from(env::var("CARGO_HOME")?).join("config.toml");
    let current_config = env::current_dir()?;
    let cargo_config = Path::new(".cargo").join("config.toml");

    for ans in current_config.ancestors() {
        println!("{:?}", ans.join(&cargo_config));
    }

    if user_config.exists() && user_config.is_dir() {
        bail!("User config is a directory");
    }

    let config: Document = fs::read_to_string(user_config)?.parse()?;


    Ok(())
}

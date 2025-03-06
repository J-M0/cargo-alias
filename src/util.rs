use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

pub fn get_config_path(local: bool) -> Result<PathBuf> {
    if local {
        let mut p = env::current_dir()?;
        p.push(".cargo/config.toml");
        Ok(p)
    } else {
        let mut p: PathBuf = env::var_os("CARGO_HOME")
            .context("`CARGO_HOME` not set")?
            .into();
        p.push("config.toml");
        Ok(p)
    }
}

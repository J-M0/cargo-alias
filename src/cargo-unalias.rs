mod util;

use anyhow::{Context, Result};
use clap::{Args, Parser};
use std::fs;
use toml_edit::{DocumentMut, Item};
use util::get_config_path;

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum Cargo {
    Unalias(Opt),
}

#[derive(Args)]
#[command(about = "Delete cargo aliases", version)]
struct Opt {
    /// Use local `.cargo/config.toml`
    #[arg(short, long)]
    local: bool,
    /// Name of alias to delete
    alias: String,
}

fn main() -> Result<()> {
    let Cargo::Unalias(opt) = Cargo::parse();

    let config_path = get_config_path(opt.local)?;

    let mut config: DocumentMut = fs::read_to_string(&config_path)?.parse()?;

    if let Item::Table(ref mut t) = config["alias"] {
        t.remove(&opt.alias)
            .with_context(|| format!("alias `{}` not found", opt.alias))?;
        fs::write(&config_path, config.to_string())?;
    }

    Ok(())
}

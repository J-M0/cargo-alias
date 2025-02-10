mod util;

use clap::{Args, Parser};
use std::fs;
use toml_edit::DocumentMut;
use util::CARGO_HOME;

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum Cargo {
    Unalias(Opt),
}

#[derive(Args)]
#[command(about = "Delete cargo aliases", version)]
struct Opt {
    /// Name of alias to delete
    alias: String,
}

fn main() -> anyhow::Result<()> {
    let Cargo::Unalias(opt) = Cargo::parse();

    let mut config: DocumentMut = fs::read_to_string(CARGO_HOME.as_path())?.parse()?;

    if config.contains_table("alias") {
        config["alias"].as_table_mut().unwrap().remove(&opt.alias);
        fs::write(CARGO_HOME.as_path(), config.to_string())?;
    }

    Ok(())
}

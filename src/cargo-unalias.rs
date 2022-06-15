mod util;

use clap::Parser;
use std::fs;
use toml_edit::Document;
use util::CARGO_HOME;

#[derive(Parser)]
#[clap(name = "cargo", bin_name = "cargo")]
enum Cargo {
    Unalias(Opt),
}

#[derive(clap::Args)]
#[clap(about = "Delete cargo aliases", version)]
struct Opt {
    /// Name of alias to delete
    alias: String,
}

fn main() -> anyhow::Result<()> {
    let Cargo::Unalias(opt) = Cargo::parse();

    let mut config: Document = fs::read_to_string(&*CARGO_HOME)?.parse()?;

    if config.contains_table("alias") {
        config["alias"].as_table_mut().unwrap().remove(&opt.alias);
        fs::write(&*CARGO_HOME, config.to_string())?;
    }

    Ok(())
}

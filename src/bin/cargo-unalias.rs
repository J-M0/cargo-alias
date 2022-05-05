use clap::Parser;
use std::env;
use std::fs;
use std::path::PathBuf;
use toml_edit::Document;

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

    let cargo_home_config = PathBuf::from(env::var("CARGO_HOME")?).join("config.toml");

    let mut config: Document = fs::read_to_string(&cargo_home_config)?.parse()?;
    config["alias"].as_table_mut().unwrap().remove(&opt.alias);
    fs::write(cargo_home_config, config.to_string_in_original_order())?;

    Ok(())
}

use std::env;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use toml_edit::Document;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo unalias", about = "Delete cargo aliases")]
struct Opt {
    /// Name of alias to delete
    alias: String,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_iter(env::args_os().filter(|arg| arg != "unalias"));

    let cargo_home_config = PathBuf::from(env::var("CARGO_HOME")?).join("config.toml");

    let mut config: Document = fs::read_to_string(&cargo_home_config)?.parse()?;
    config["alias"].as_table_mut().unwrap().remove(&opt.alias);
    fs::write(cargo_home_config, config.to_string_in_original_order())?;

    Ok(())
}

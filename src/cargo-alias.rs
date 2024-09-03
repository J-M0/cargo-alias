mod util;

use anyhow::bail;
use clap::{Args, Parser};
use std::fs;
use toml_edit::{Document, Value};
use util::CARGO_HOME;

#[derive(Parser)]
#[clap(name = "cargo", bin_name = "cargo")]
enum Cargo {
    Alias(Opt),
}

#[derive(Args)]
#[clap(about = "Create and view cargo aliases", version)]
struct Opt {
    /// Alias to define. Should be in the form name='command list'
    alias: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let Cargo::Alias(opt) = Cargo::parse();

    let mut config = match fs::read_to_string(CARGO_HOME.as_path()) {
        Ok(string) => string.parse()?,
        Err(_) => Document::new(),
    };

    if !config.contains_table("alias") {
        config["alias"] = toml_edit::table();
    }

    if let Some(new_alias) = opt.alias {
        let (alias, commands) = new_alias.split_once('=').unwrap();
        config["alias"][&alias] = toml_edit::value(commands);
        fs::write(CARGO_HOME.as_path(), config.to_string())?;
    } else {
        print_aliases(config)?;
    }

    Ok(())
}

fn print_aliases(config: Document) -> anyhow::Result<()> {
    // None of the unwraps here should ever fail because
    // cargo will validate the config and complain
    // before we even get to run.
    for alias in config["alias"].as_table().unwrap().iter() {
        let (alias_name, val) = alias;

        let val = match val.as_value().unwrap() {
            Value::String(s) => s.value().into(),
            Value::Array(a) => a
                .iter()
                .map(|i| i.as_str().unwrap())
                .collect::<Vec<&str>>()
                .join(" "),
            _ => bail!("value of {} is not a list or string", alias_name),
        };

        println!("cargo alias {}='{}'", alias, val);
    }

    Ok(())
}

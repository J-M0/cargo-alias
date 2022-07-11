mod util;

use anyhow::bail;
use clap::Parser;
use std::fs;
use toml_edit::{value, Document, Item, Value};
use util::CARGO_HOME;

#[derive(Parser)]
#[clap(name = "cargo", bin_name = "cargo")]
enum Cargo {
    Alias(Opt),
}

#[derive(clap::Args)]
#[clap(about = "Create and view cargo aliases", version)]
struct Opt {
    /// Alias to define. Should be in the form name='command list'
    alias: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let Cargo::Alias(opt) = Cargo::parse();

    let mut config: Document = match fs::read_to_string(&*CARGO_HOME) {
        Ok(string) => string.parse()?,
        Err(_) => Document::new(),
    };

    if !config.contains_table("alias") {
        config["alias"] = toml_edit::table();
    }

    if let Some(new_alias) = opt.alias {
        let (alias, commands) = new_alias.split_once("=").unwrap();
        config["alias"][&alias] = value(commands);
        fs::write(&*CARGO_HOME, config.to_string())?;
    } else {
        print_aliases(config)?;
    }

    Ok(())
}

fn print_aliases(config: Document) -> anyhow::Result<()> {
    for alias in config["alias"].as_table().unwrap().iter() {
        let (alias_name, val) = alias;

        let val = match val {
            Item::Value(v) => v,
            _ => bail!("value of {} must be a list or string", alias_name),
        };

        let val = match val {
            Value::String(s) => s.value().into(),
            Value::Array(a) => a
                .iter()
                .map(|i| i.as_str().unwrap())
                .collect::<Vec<&str>>()
                .join(" "),
            _ => bail!("value of {} is not a list or string", alias_name),
        };

        println!("alias {}='{}'", alias_name, val);
    }

    Ok(())
}

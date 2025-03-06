mod util;

use anyhow::bail;
use clap::{Args, Parser};
use std::fs;
use std::io;
use toml_edit::{DocumentMut, Table, Value};
use util::get_config_path;

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum Cargo {
    Alias(Opt),
}

#[derive(Args)]
#[command(about = "Create and view cargo aliases", version)]
struct Opt {
    /// Use local `.cargo/config.toml`
    #[arg(short, long)]
    local: bool,
    /// Alias to define. Should be in the form name='command list'
    alias: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let Cargo::Alias(opt) = Cargo::parse();

    let config_path = get_config_path(opt.local)?;

    let mut config = match fs::read_to_string(&config_path) {
        Ok(string) => string.parse()?,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => DocumentMut::new(),
            _ => return Err(e.into()),
        },
    };

    let aliases = config
        .entry("alias")
        .or_insert_with(toml_edit::table)
        .as_table_mut()
        .unwrap();

    if let Some(new_alias) = opt.alias {
        let (alias, commands) = new_alias.split_once('=').unwrap();
        aliases[alias] = toml_edit::value(commands);
        fs::write(&config_path, config.to_string())?;
    } else {
        print_aliases(aliases)?;
    }

    Ok(())
}

fn print_aliases(aliases: &Table) -> anyhow::Result<()> {
    // None of the unwraps here should ever fail because
    // cargo will validate the config and complain
    // before we even get to run.
    for (name, value) in aliases {
        let value = match value.as_value().unwrap() {
            Value::String(s) => s.value(),
            Value::Array(a) => &a
                .iter()
                .map(|i| i.as_str().unwrap())
                .collect::<Vec<&str>>()
                .join(" "),
            _ => bail!("value of {name} is not a list or string"),
        };

        println!("cargo alias {name}='{value}'");
    }

    Ok(())
}

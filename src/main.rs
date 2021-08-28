use anyhow::bail;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::{Document, Item, Value, value};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo alias", about = "Create and view cargo aliases")]
struct Opt {
    /// Alias to define. Should be in the form name='command list'
    alias: Option<String>
}

fn main() -> anyhow::Result<()> {
    let user_config = PathBuf::from(env::var("CARGO_HOME")?).join("config.toml");
    let current_config = env::current_dir()?;
    let cargo_config = Path::new(".cargo").join("config.toml");

    let opt = Opt::from_args();

    // for ans in current_config.ancestors() {
    //     println!("{:?}", ans.join(&cargo_config));
    // }

    if user_config.exists() && user_config.is_dir() {
        bail!("User config is a directory");
    }

    let mut config: Document = fs::read_to_string(&user_config)?.parse()?;

    if let Some(new_alias) = opt.alias {
        let (alias, commands) = new_alias.split_once("=").unwrap();
        config["alias"][&alias]  = value(commands);
        fs::write(user_config, config.to_string_in_original_order())?;
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
        match val {
            Value::String(_) => println!("alias {}='{}'", alias_name, val.as_str().unwrap()),
            Value::Array(_) => {
                let val = val
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|i| i.as_str().unwrap())
                    .collect::<Vec<&str>>()
                    .join(" ");
                println!("alias {}='{}'", alias_name, val);
            }
            _ => bail!("value of {} is not a list or string", alias_name),
        }
    }

    Ok(())
}

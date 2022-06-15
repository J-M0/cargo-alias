use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

lazy_static! {
    pub static ref CARGO_HOME: PathBuf =
        PathBuf::from(env::var("CARGO_HOME").unwrap()).join("config.toml");
}

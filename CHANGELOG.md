# Changelog

## v0.2.2

### Bugs fixed

- Panic when `$CARGO_HOME/config.toml` doesn't exist

## v0.2.1

### Bugs fixed

- Panic with empty config.toml (thanks @IkaR49!) #1
- Crashes when run as a cargo subcommand #2

### Dependencies updated

- structopt replaced with clap 3
- toml_edit updated to 0.14.4

## v0.2.0

### Changes

- `cargo alias` now prints aliases in a reusable form

### Bugs Fixed

- Fixed a crash when creating an alias `config.toml` does not exist

## v0.1.0

Initial release

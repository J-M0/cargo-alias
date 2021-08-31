# cargo-alias

Easily create, view, and delete [cargo aliases](https://doc.rust-lang.org/cargo/reference/config.html#alias).

## Installation

Simply run

```commandline
cargo install cargo-alias
```

to install.

## Usage

Use `cargo alias` to print all defined aliases

Use `cargo alias name='command list'` to make a new alias

Use `cargo unalias` to delete an alias

## Examples

Create an alias to list binaries installed by cargo:

```commandline
cargo alias installed='install --list'
```

## Current Status and Future

Currently `cargo-alias` only supports aliases defined in `$CARGO_HOME`.

Future plans:

- Support for managing aliases in local configs
- Code quality cleanup
- Optimizations
- Better documentation

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

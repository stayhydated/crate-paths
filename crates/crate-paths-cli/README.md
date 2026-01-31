# crate-paths-cli

A Cargo subcommand to generate path trees from Rust crates.

## Installation

```bash
cargo install crate-paths-cli
```

## Usage

Generate a path tree for a crate:

```bash
cargo crate-paths --crate-name std --output-path ./std_paths.rs
```

### Output path

- If `--output-path` has a file extension, writes directly to that file.
- If no extension, treats it as a directory and writes `{crate_name_snake_case}.rs` within it.

```bash
# Writes to ./std_paths.rs
cargo crate-paths --crate-name std --output-path ./std_paths.rs

# Writes to ./generated/std.rs
cargo crate-paths --crate-name std --output-path ./generated

# Writes to ./generated/http_body.rs
cargo crate-paths --crate-name http-body --output-path ./generated
```

### Backends

- **Auto (default)**: tries `rustup` -> `local` -> `docs.rs`.
- **`--backend rustup`**: use rustup stdlib docs.
- **`--backend local`**: analyze a crate available in `target/doc` (typically a workspace member or dependency).
- **`--backend docsrs`**: fetch `all.html` from docs.rs (network required).

### Crate version (docs.rs)

Defaults to `latest`.

```bash
cargo crate-paths --crate-name serde --crate-version 1.0.196 --backend docsrs --output-path ./serde.rs
```

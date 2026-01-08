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

### Output Path

- If `--output-path` has a file extension, writes directly to that file.
- If no extension, treats it as a directory and writes to `{crate_name}.rs` within it.

```bash
# Writes to ./std_paths.rs
cargo crate-paths --crate-name std --output-path ./std_paths.rs

# Writes to ./generated/std.rs
cargo crate-paths --crate-name std --output-path ./generated
```

### Backends

- **Auto (default)**: Cycles through the backends to find the crate. going from `rustup` -> `local` -> `docsrs`.
- **`--backend rustup`**: Force usage of Rustup source (for std lib).
- **`--backend local`**: Analyze a local crate in the workspace.
- **`--backend docsrs`**: Fetch metadata from docs.rs.

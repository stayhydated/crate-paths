# crate-paths-cli

A Cargo subcommand to generate path trees from Rust crates.

## Installation

```bash
cargo install crate-paths-cli
```

## Usage

Generate a path tree for a crate:

```bash
cargo crate-paths --crate-name std --output-path ./paths
```

### Backends
- **Auto (default)**: Cycles through the backends to find the crate. going from `rustup` -> `local` -> `docsrs`.
- **`--backend rustup`**: Force usage of Rustup source (for std lib).
- **`--backend local`**: Analyze a local crate in the workspace.
- **`--backend docsrs`**: Fetch metadata from docs.rs.

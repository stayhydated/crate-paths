# crate-paths

[![Build Status](https://github.com/stayhydated/crate-paths/actions/workflows/ci.yml/badge.svg)](https://github.com/stayhydated/crate-paths/actions/workflows/ci.yml)
[![Docs](https://docs.rs/crate-paths/badge.svg)](https://docs.rs/crate-paths/)
[![Crates.io](https://img.shields.io/crates/v/crate-paths.svg)](https://crates.io/crates/crate-paths)

Type-safe path constants for Rust code generation.

`crate-paths` gives you:
- `crate_paths::Path`, a `const`-friendly path type that implements `ToTokens`.
- A CLI that generates a module tree mirroring a crate's public items.
- Optional procedural macros for inline, compile-time-validated path constants.

## Installation

### CLI

```bash
cargo install crate-paths-cli
```

### Library

```toml
[dependencies]
crate-paths = "*"
```

## Quickstart

1) Generate a path file

```bash
cargo crate-paths --crate-name serde --output-path ./generated
```

2) Use the generated constants in a macro

```rust
use generated::serde::{Deserialize, Serialize};

let tokens = quote! {
    #[derive(#Serialize, #Deserialize)]
    struct Foo;
};
```

## Output example

```rust
// generated/serde.rs
pub const Deserialize: crate_paths::Path = crate_paths::Path::new("serde::Deserialize");
pub const Serialize: crate_paths::Path = crate_paths::Path::new("serde::Serialize");
```

Note: the crate prefix in generated paths is the snake_case form of the crate name (for example,
`http-body` becomes `http_body`).

## Optional macros

Enable the feature on `crate-paths` or depend on `crate-paths-macros` directly.

```toml
[dependencies]
crate-paths = { version = "*", features = ["macros"] }
```

```rust
use crate_paths::{path, path_val};

path!(std::sync::Arc);
const MY_PATH: crate_paths::Path = path_val!(std::collections::HashMap);
```

## Backends

The CLI can source item lists from multiple places:
- **rustup**: reads stdlib docs from `rustup doc --path`.
- **local**: runs `cargo doc --workspace` and parses `target/doc/<crate>/all.html`.
- **docs.rs**: fetches `all.html` from docs.rs (network required).

The default mode auto-tries `rustup` -> `local` -> `docs.rs`.

## Crates

| Crate | Purpose |
| --- | --- |
| `crate-paths` | Runtime `Path` type and `ToTokens` impl. |
| `crate-paths-macros` | Optional procedural macros (`path!`, `path_val!`). |
| `crate-paths-cli` | `cargo crate-paths` binary. |
| `crate-paths-cli-core` | Backend fetching, parsing, tree building. |

## Development

Common tasks via `just`:
- `just fmt`
- `just test`
- `just clippy`
- `just test-docs`

## License

Dual-licensed under MIT or Apache-2.0.

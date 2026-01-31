# Project Overview

`crate-paths` is a tool for **generating type-safe path constants** for Rust code generation. It focuses on:

1. **Type Safety**: Providing `syn::Path`-compatible constants that are checked at compile time.
1. **Mirroring**: Automatically generating a module structure that mirrors the target crate.
1. **Developer Experience**: A simple CLI (`crate-paths-cli`) that integrates easily into build workflows.

## Architecture Documentation Index

| Crate | Link to Architecture Doc | Purpose |
| --- | --- | --- |
| **Core** | | |
| `crate-paths` | [Architecture](crates/crate-paths/docs/ARCHITECTURE.md) | Runtime `Path` type and `ToTokens` implementation. |
| `crate-paths-macros` | [Architecture](crates/crate-paths-macros/docs/ARCHITECTURE.md) | Optional procedural macros for inline path constants. |
| **CLI Tool** | | |
| `crate-paths-cli` | [Architecture](crates/crate-paths-cli/docs/ARCHITECTURE.md) | Primary developer-facing CLI (`cargo crate-paths`). |
| **Tooling Internals** | | |
| `crate-paths-cli-core` | [Architecture](crates/crate-paths-cli-core/docs/ARCHITECTURE.md) | Core logic for scraping, parsing, and code generation. |

## Crate Descriptions

### Core Layers

- **`crate-paths`**: The user-facing library. Defines `Path` and implements `ToTokens` so generated constants can be used directly in `quote!`.
- **`crate-paths-macros`**: Provides `path!` and `path_val!` for compile-time validated path constants.

### CLI Tool

- **`crate-paths-cli`**: The Cargo subcommand that fetches crate docs, generates the path tree, and writes the output file.

### Tooling Internals

- **`crate-paths-cli-core`**: Backend fetching (rustup/local/docs.rs), HTML parsing, item modeling, and module tree rendering.

## Development

- **Rust**: Use `cargo` for building, testing, and running Rust code.
- **Task runner**: Use `just` for common commands (fmt, test, clippy, docs).
- **Testing**: Use [insta](https://insta.rs/) for snapshot tests where appropriate.

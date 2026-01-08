# Project Overview

`crate-paths` is a tool for **generating type-safe path constants** for Rust code generation. It focuses on:

1. **Type Safety**: Providing `syn::Path`-compatible constants that are checked at compile time.
1. **Mirroring**: Automatically generating a module structure that mirrors the target crate.
1. **Developer Experience**: A simple CLI (`crate-paths-cli`) that integrates easily into build workflows.

## Architecture Documentation Index

| Crate | Link to Architecture Doc | Purpose |
|-------|-------------------|---------|
| `crate-paths` | [Architecture](crates/crate-paths/docs/ARCHITECTURE.md) | Runtime library and `Path` definition. |
| `crate-paths-macros` | [Architecture](crates/crate-paths-macros/docs/ARCHITECTURE.md) | Optional procedural macros. |
| `crate-paths-cli` | [Architecture](crates/crate-paths-cli/docs/ARCHITECTURE.md) | The `cargo crate-paths` command-line tool. |
| `crate-paths-cli-core` | [Architecture](crates/crate-paths-cli-core/docs/ARCHITECTURE.md) | Core logic for scraping and generation. |

## Crate Descriptions

### Core Layers

- **`crate-paths`**: The required runtime library. It defines the `Path` struct and implements `ToTokens`, allowing constants to be used directly in `quote!`.
- **`crate-paths-macros`**: An optional library providing procedural macros for the ecosystem.

### Tooling

- **`crate-paths-cli`**: The binary installed by users. It serves as the entry point for generating path files.
- **`crate-paths-cli-core`**: The library containing the core logic for the CLI. It handles crate analysis, documentation scraping, and code generation.

## Development

- **Rust**: Use `cargo` for building, testing, and running Rust code.
- **Testing**: Use [insta](https://insta.rs/) for snapshot tests where appropriate.

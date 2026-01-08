# crate-paths

[![Build Status](https://github.com/stayhydated/crate-paths/actions/workflows/ci.yml/badge.svg)](https://github.com/stayhydated/crate-paths/actions/workflows/ci.yml)
[![Docs](https://docs.rs/crate-paths/badge.svg)](https://docs.rs/crate-paths/)
[![Crates.io](https://img.shields.io/crates/v/crate-paths.svg)](https://crates.io/crates/crate-paths)

**Tools for generating type-safe path constants for Rust code generation.**

The primary tool in this ecosystem is the **CLI**, which generates a mirroring module structure for any crate, containing `syn::Path`-compatible constants.

## Workflow

1. **Install the CLI**

   ```bash
   cargo install crate-paths-cli
   ```

1. **Generate Paths**
   Run the CLI against a crate (e.g., `serde`) to generate a path definition file.

   ```bash
   cargo crate-paths --crate-name serde --output-path ./serde_paths.rs
   ```

1. **Output (Example)**
   The generated code mirrors the crate's structure, providing `crate_paths::Path` constants that can be used directly in `quote!`.

   ```rust
   // ./serde_paths.rs
   pub const Deserialize: crate_paths::Path = crate_paths::Path::new("serde::Deserialize");
   pub const Serialize: crate_paths::Path = crate_paths::Path::new("serde::Serialize");

   // ...

   ```

1. **Usage in Procedural Macros**
   Use the generated paths in your `quote!` macros. They implement `ToTokens` and work in `const` contexts!

   ```rust
   use serde_paths::{Deserialize, Serialize};

   let tokens = quote! {
       #[derive(#Serialize, #Deserialize)]
       struct Foo;
   };
   ```

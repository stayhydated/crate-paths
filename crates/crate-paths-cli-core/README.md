# crate-paths-cli-core

Core logic for the `crate-paths` CLI ecosystem. This crate is primarily intended for internal use by `crate-paths-cli`, but it can be used directly to build custom tooling.

## What it provides

- Backends for fetching rustdoc item lists (rustup, local, docs.rs).
- HTML parsing to extract items from `all.html`.
- Module tree generation and Rust code rendering.

## Usage

Most users should install and run the CLI:

```bash
cargo install crate-paths-cli
```

If you need direct access to the library, add it as a dependency and call the backend `process` functions, then render with `ModTree`.

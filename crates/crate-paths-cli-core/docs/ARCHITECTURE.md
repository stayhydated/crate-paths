# Architecture

## Purpose
`crate-paths-cli-core` owns the core logic for discovering items in a crate and converting them into a Rust module tree of `crate_paths::Path` constants. It is designed to be reused by multiple frontends (CLI or future tools).

## Responsibilities
- Fetch crate documentation from one of several backends.
- Parse `all.html` into a list of items.
- Build a module tree that mirrors the crate structure.
- Render Rust code for the tree (optionally formatted).

## Module map
- `backend/`: acquisition strategies.
  - `docrs`: downloads `all.html` from docs.rs (supports zstd encoding).
  - `local`: runs `cargo doc --workspace` and reads `target/doc/<crate>/all.html`.
  - `rustup`: reads stdlib docs from `rustup doc --path`.
- `parser`: HTML scraper that extracts item paths and kinds.
- `item` / `item_kind`: normalized representation of extracted items.
- `tree`: builds a module tree and renders output.
- `error`: top-level error type that wraps backend and parser failures.

## Data flow
1. Backend loads `all.html`.
2. `parser::parse_html_to_items` extracts `ItemEntry` records.
3. `tree::ModTree` groups items into modules and renders Rust code.
4. Output is returned to the caller (CLI writes it to disk).

## Rendering details
- Items are grouped by module path (split on `::`).
- Each item is rendered as a `pub const` with `crate_paths::Path::new("<crate_name_snake_case>::path")`.
- The final output is passed through `rustfmt` when available; on failure it falls back to the raw string.

## Error handling
- `CoreError` wraps `BackendError` and `ParserError`.
- Backend errors capture IO, HTTP, and command failures.
- Parser errors indicate malformed HTML or missing content.

## Extension points
- Add a new backend by implementing `process` and wiring it into `backend/mod.rs`.
- Extend `item_kind` if new rustdoc kinds appear in `all.html`.
- Adjust the selector in `parser` if rustdoc HTML structure changes.

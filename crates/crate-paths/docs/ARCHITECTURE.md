# Architecture

## Purpose

`crate-paths` provides a small, `const`-friendly path type that can be used in Rust code generation. The goal is to create `syn::Path`-compatible constants that can be passed directly into `quote!` and other token-building APIs.

## Core type

- `Path<'a>`: wraps a `&'a str` and represents a fully-qualified Rust path (for example, `std::sync::Arc`).
- `Path::new`: `const fn` constructor so values can live in `const` contexts.
- `quote::ToTokens` implementation: parses the string into `syn::Path` at token emission time and forwards to `syn`'s `ToTokens`.

## Data flow

1. User creates a `Path` value (typically via the CLI output or macros).
1. The `Path` is passed to a macro or `quote!` invocation.
1. `ToTokens` parses the stored string into `syn::Path` and emits tokens.

## Invariants and constraints

- The stored string must parse as a valid Rust path. Invalid values will panic during tokenization (`syn::parse_str` fails).
- `Path` is intentionally minimal: it stores only the string form and delegates parsing to `syn`.

## Feature flags

- `macros`: re-exports the `crate-paths-macros` procedural macros for convenience.

## Extension points

- Additional constructors or validation helpers can be added here if the project needs eager validation or richer metadata.

# Architecture

## Purpose
`crate-paths-macros` provides procedural macros that turn Rust paths into `crate_paths::Path` values. The macros perform a compile-time validity check by referencing the provided path.

## Macros
### `path!`
- **Input**: a Rust path (for example, `std::sync::Arc`).
- **Output**: a `pub const` whose identifier is the final path segment (`Arc`).
- **Validation**: emits `use <path> as _;` inside the expansion to ensure the item exists.
- **Keyword handling**: if the last segment is a Rust keyword, the macro emits a raw identifier (for example, `r#type`).

### `path_val!`
- **Input**: the same path syntax as `path!`.
- **Output**: an expression that evaluates to `crate_paths::Path::new("...")`.
- **Use case**: lets consumers choose their own constant name or embed in other code.

## Implementation details
- Parses the input using `syn::Path`.
- Builds a `::`-joined string from the path segments.
- Uses `quote` to construct the expansion and `check_keyword` to produce safe identifiers.

## Constraints
- The input must be a syntactic path. Arbitrary expressions are not supported.
- Validation only guarantees that the path resolves during compilation of the calling crate.

## Extension points
- Additional helper macros can be added here if new creation patterns are needed.

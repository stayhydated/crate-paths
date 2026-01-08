# Architecture

## Overview

`crate-paths-macros` provides the procedural macros used by `crate-paths`. It allows users to define paths in a compile-time safe manner.

## Macros

### `path!`

- **Input**: A Rust path (e.g., `std::sync::Arc`).
- **Output**: Defines a `pub const` of type `crate_paths::Path`.
- **Validation**: Generates code that performs a "use" check (`use ... as _`) to ensure the path actually blocks compilation if it's invalid.

### `path_val!`

- **Input**: Same as `path!`.
- **Output**: Returns the `crate_paths::Path` expression directly (for assignment to consumers' own constants).

## Design Pattern

The macros use the "compile-fail test" trick where they emit code that references the item to ensure it exists, but then wrap the runtime representation in the safe `Path` struct.

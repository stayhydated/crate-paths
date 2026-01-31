# crate-paths-macros

Procedural macros for creating `crate_paths::Path` constants with compile-time validation.

## Installation

```toml
[dependencies]
crate-paths-macros = "*"
crate-paths = "*"
```

## Usage

### `path!`

Creates a public constant named after the last segment of the path.

```rust
use crate_paths_macros::path;

path!(std::sync::Arc);
// expands to: pub const Arc: crate_paths::Path = ...
```

### `path_val!`

Returns a `crate_paths::Path` expression so you can choose your own name.

```rust
use crate_paths_macros::path_val;

const MAP: crate_paths::Path = path_val!(std::collections::HashMap);
```

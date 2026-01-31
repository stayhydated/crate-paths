# crate-paths

Library for defining and using a `syn::Path`-like representation that implements `ToTokens`.

## Installation

```toml
[dependencies]
crate-paths = "*"
```

## Usage

```rust
use crate_paths::Path;

pub const SERIALIZE: Path = Path::new("serde::Serialize");

let tokens = quote! {
    #[derive(#SERIALIZE)]
    struct Example;
};
```

## Optional macros

Enable the `macros` feature to re-export `path!` and `path_val!`.

```toml
[dependencies]
crate-paths = { version = "*", features = ["macros"] }
```

```rust
use crate_paths::{path, path_val};

path!(std::sync::Arc);
const MY_PATH: crate_paths::Path = path_val!(std::collections::HashMap);
```

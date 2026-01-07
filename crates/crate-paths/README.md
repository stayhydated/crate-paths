Crate that export the `Path` type, necessary for the generated code by [crate-paths-cli](../crate-paths-cli).
This type implements the `ToTokens` trait. so you can use it like this:

```rs
use std_crate_paths::sync::Arc;

let tokens = quote! {
    let a = #Arc::new("b");
};
```

which will expand to:

```rs
let a = std::sync::Arc::new("b");
```

Optionally, exposes macros from [crate-paths-macro](../crate-paths-macros/README.md) when the `macro` feature is enabled

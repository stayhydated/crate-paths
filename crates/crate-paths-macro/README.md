## path!(...);
Usage:
```rust
path!(std::sync::Arc);
```

This expands to
```rust
#[allow(non_upper_case_globals)]
pub const Arc: crate_paths::Path = {
    #[allow(unused_imports)]
    use std::sync::Arc as _;

    crate_paths::Path::new("std::sync::Arc")
};
```

## path_val!(...);
Usage:

```rust
path_val!(std::sync::Arc);
```

This expands to:

```rust
{
  #[allow(unused_imports)]
  use std::sync::Arc as _;

  crate_paths::Path::new("std::sync::Arc")
}
```

Which you would later assign to a constant:

```rust
const SOME_CONSTANT: crate_paths::Path = path_val!("std::sync::Arc");
```

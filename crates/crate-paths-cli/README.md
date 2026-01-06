tired of doing "something" like this?

```rs
let struct_name = get();
let field_name = get();
let field_type = get();

let tokens = quote! {
    #[derive(Clone, serde::Serialize, serde::Deserialize)]
    pub struct #struct_name {
        #field_name: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<String, #field_type>>>,
        metrics: std::sync::Arc<std::sync::atomic::AtomicU64>,
    }

    impl #struct_name {
        pub fn new() -> Self {
            Self {
                #field_name: std::sync::Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
                metrics: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0)),
            }
        }
    }
};
```

This cli will generate definitions that will allow you to do:

```rs
let struct_name = get();
let field_name = get();
let field_type = get();

use serde_crate_paths::{Serialize, Deserialize};
use std_crate_paths::sync::{Arc, RwLock};
use std_crate_paths::sync::atomic::AtomicU64;
use std_crate_paths::collections::HashMap;

let tokens = quote! {
    #[derive(Clone, #Serialize, #Deserialize)]
    pub struct #struct_name {
        #field_name: #Arc<#RwLock<#HashMap<String, #field_type>>>,
        metrics: #Arc<#AtomicU64>,
    }

    impl #struct_name {
        pub fn new() -> Self {
            Self {
                #field_name: #Arc::new(#RwLock::new(#HashMap::new())),
                metrics: #Arc::new(#AtomicU64::new(0)),
            }
        }
    }
};
```

## Examples

See a basic example in [example](../../example), **whose Justfile contains information about the backend used.**

# crate-paths

Library for defining and using a `syn::Path`-like representation that implements `ToTokens`.

It serves as a `syn::Path`-like representation, primarily because it can be constructed and used in a `const` context, which `syn::Path` cannot.
# Architecture

## Overview

`crate-paths` is a lightweight wrapper crate designed to bridge the gap between `syn::Path` and `quote::ToTokens` in a way that allows for easier code generation of paths.

It serves as a `syn::Path`-like representation, primarily because it can be constructed and used in a `const` context, which `syn::Path` cannot.

## Design

The core design revolves around the `Path` struct, which wraps a string representation of a path (e.g., `std::sync::Arc`). This struct implements `ToTokens`, parsing the string into a `syn::Path` at macro expansion time.

### Key Components

- **`Path<'a>`**: The main struct. Wraps a `&'a str`.
- **`ToTokens` impl**: Parses the string into `syn::Path` and emits it.

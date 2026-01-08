# Architecture

## Overview

`crate-paths-cli-core` contains the shared business logic for the `crate-paths` CLI tools. It implements the "backends" responsible for resolving crate items and generating the path tree.

## Modules

### `backend`

Handles the different strategies for retrieving crate information:

- **`rustup`**: Fetches standard library paths from the Rust toolchain.
- **`local`**: Analyzes crates within the current workspace.
- **`docsrs`**: Fetches crate information from docs.rs (likely via online documentation parsing or metadata).

### `item` & `item_kind`

Defines the internal representation of Rust language items (structs, enums, functions, modules) and their types.

### `tree`

Responsible for constructing the hierarchy of paths from the flat list of items.

## Usage

This crate is primarily consumed by `crate-paths-cli` but is designed to be reusable for other tools needing similar functionality.

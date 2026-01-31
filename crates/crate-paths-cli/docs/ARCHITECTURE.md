# Architecture

## Purpose
`crate-paths-cli` is the user-facing binary (`cargo crate-paths`). It handles argument parsing, backend selection, and writing generated code to disk. All heavy lifting happens in `crate-paths-cli-core`.

## Components
- `main.rs`: clap-based CLI definitions and output path resolution.
- `backend/mod.rs`: runs a chosen backend or auto-fallback sequence.
- `writer.rs`: writes generated output to the target file.
- `consts.rs`: build-time constants (for example, the user-agent string).
- `error.rs`: error aggregation for CLI failures.

## Output path rules
- If `--output-path` has a file extension, the CLI writes directly to that file.
- If it has no extension, the CLI treats it as a directory and writes `{crate_name_snake_case}.rs` inside it.

## Backend selection
- `--backend rustup`: for stdlib crates via `rustup doc --path`.
- `--backend local`: uses `cargo doc --workspace` and local docs.
- `--backend docsrs`: fetches from docs.rs.
- `--crate-version` applies to the docs.rs backend (default `latest`).
- Default behavior (`run_auto`): tries `rustup` -> `local` -> `docsrs`, and uses `cargo metadata`
  to pick a docs.rs version when available (otherwise falls back to `latest`).

## Data flow
1. Parse CLI arguments.
2. Resolve output path.
3. Invoke backend (auto or explicit) via `crate-paths-cli-core`.
4. Render module tree and write file.

## Failure modes
- Missing tools (`rustup`, `cargo`) or network issues can cause backend failures.
- If `rustfmt` is missing, rendering falls back to the unformatted output.
- The auto runner logs failures and proceeds to the next backend until one succeeds.

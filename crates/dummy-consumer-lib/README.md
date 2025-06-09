Dummy lib that consumes `gpui`, since currently `gpui` is not yet published to crates.io

This way, `cargo doc` will generate docs for it

Crates have to be declared in the `[dependencies]` block, won't work if declared in `[dev-dependencies]`

```toml
[dependencies]
gpui = { git = "https://github.com/zed-industries/zed.git", rev = "06a199da4d8a147c7867f4c82ffafed4974bea24" }
```

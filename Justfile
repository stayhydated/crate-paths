set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

default:
    @just --list

fmt:
    cargo sort-derives
    cargo fmt
    taplo fmt
    uvx mdformat .

test:
    cargo test
    cd tests && just all

test-publish:
    cargo publish --workspace --dry-run --allow-dirty

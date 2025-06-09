set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

default:
    @just --list

fmt:
    cargo sort -w
    cargo sort-derives
    cargo fmt

test:
  cargo test
  cd tests-paths && just all

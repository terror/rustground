export RUST_LOG := 'info'

alias f := fmt

default:
  just --list

all: build test clippy fmt-check

build:
  cargo build

clippy:
  cargo clippy --all-targets --all-features

container:
  docker-compose up -d

fmt:
  cargo fmt
  prettier --write .
  prettier --write client/**/*.svelte

fmt-check:
  cargo fmt --all -- --check
  @echo formatting check done

run *args:
  cargo run -- {{args}}

test:
  cargo test

serve:
  cargo run -- --source=crates.json serve

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"

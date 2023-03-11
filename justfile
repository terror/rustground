export RUST_LOG := 'info'

alias f := fmt

default:
  just --list

all: build test clippy fmt-check

build:
  cargo build

client:
  npm run dev

clippy:
  cargo clippy --all-targets --all-features

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

server:
  cargo run -- --source=crates.json serve

services:
  docker-compose up -d

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"

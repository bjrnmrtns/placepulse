#!/usr/bin/env bash
cargo check && 
cargo fmt -- --check &&
cargo test &&
cargo clippy &&
true


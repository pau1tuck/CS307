#!/bin.bash

rustup update
rustup install nightly
rustup show
rustup default stable

cargo new
cargo metadata
cargo run
cargo build --release
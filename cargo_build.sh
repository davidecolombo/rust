#!/bin/bash
set -euxo pipefail

cargo build
#cargo build --release
#cargo check
#cargo run

# Intelli IDEA + Rust plugin
#cargo build --color=always --message-format=json-diagnostic-rendered-ansi --package rust_samples --bin rust_samples
#cargo run --color=always --package rust_samples --bin rust_samples

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build (native)
cargo build

# Build release
cargo build --release

# Build for WASM
cargo build --release --target wasm32-unknown-unknown

# Run an example
cargo run --example simple

# Check without building
cargo check

# Run clippy
cargo clippy

# Format code
cargo fmt
```

## Practices
- use thoughts directory as your scratchpad
- create formalized document under docs directory
- describe the design using Rust Struct and Trait to show integration between components in the doc
- use mermaid for graph, i.e. sequence diagram, flow chart, etc.
- Run cargo test and cargo run after finsih implementation
- Fix the root cause instead of fix surface symptoms, indentify the root cause and tackle it.
- Don't use //TODO, use todo!("WAHT TO BE DONE")
- Follow Rust API naming convention https://rust-lang.github.io/api-guidelines/naming.html
- Don't implment a simplfied version for simplicity, implmentation should repect to the requirement
- call cargo fmt after editing rust files
- call cargo check to verify build
- call cargo clippy after cargo check
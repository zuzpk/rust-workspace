# Zuz Rust Workspace

A modular Rust workspace for building runtime with pluggable modules.

## Overview

This repository is organized as a Cargo workspace with:

- `core` crate (`zuz`): main library and executable node binary
- `modules/shared` crate (`zuz-shared`): shared traits and contracts
- `modules/base` crate (`zuz-base`): base module implementation

The node currently boots a module registry and runs module lifecycle hooks (`init`, `start`) asynchronously using Tokio.

## Workspace Layout

```text
.
├── Cargo.toml                 # Workspace manifest
├── core/
│   ├── Cargo.toml             # `zuz` crate
│   └── src/
│       ├── lib.rs
│       ├── config.rs
│       └── bin/
│           └── node.rs        # `zuz-node` binary entrypoint
└── modules/
	├── shared/
	│   ├── Cargo.toml         # `zuz-shared` crate
	│   └── src/lib.rs         # `ZuzModule` trait
	└── base/
		├── Cargo.toml         # `zuz-base` crate
		└── src/lib.rs         # `BaseModule`
```

## Requirements

- Rust stable toolchain (recommended: latest stable)
- Cargo (installed with Rust)

Optional:

- `rustfmt` and `clippy` components for formatting and linting

## Quick Start

1. Clone the repository.
2. Build all workspace crates:

```bash
cargo build --workspace
```

3. Run the node binary:

```bash
cargo run -p zuz --bin zuz-node
```

Expected output (current implementation):

```text
BaseModule initialized!
BaseModule started!
```

## Common Commands

Build everything:

```bash
cargo build --workspace
```

Run tests for all crates:

```bash
cargo test --workspace
```

Check code without producing binaries:

```bash
cargo check --workspace
```

Format code:

```bash
cargo fmt --all
```

Lint with Clippy:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Run only the node binary:

```bash
cargo run -p zuz --bin zuz-node
```

## Module System

Modules implement the `ZuzModule` trait from `zuz-shared`:

- `name(&self) -> &str`
- `init(&self) -> Result<(), Box<dyn std::error::Error>>`
- `start(&self)`

The node entrypoint builds a module registry and executes each module in sequence.

## Adding a New Module

1. Create a new crate under `modules/<your-module>`.
2. Add it to workspace members in the root `Cargo.toml`.
3. Implement `zuz_shared::ZuzModule` for your module type.
4. Register it in `core/src/bin/node.rs`.

Example registration pattern:

```rust
let registry: Vec<Arc<dyn ZuzModule>> = vec![
	Arc::new(BaseModule),
	// Arc::new(YourModule),
];
```

## Versioning

The runtime version is sourced from Cargo package metadata:

```rust
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
```

Update version in `[workspace.package]` in the root `Cargo.toml`.

## License

MIT

# BECOMPOSE Showcase

A comprehensive showcase of BECOMPOSE UI components, demonstrating both basic components and Material UI components.

## Features

- **Basic Components**: Text, Button, Column, Row, Surface
- **Material UI Components**: (Coming soon)
- **Cross-platform**: Runs on desktop (native) and web (WASM)

## Building and Running

### Native (Desktop)

```bash
# Build
cargo build --release

# Run
cargo run --example showcase
```

### Web (WASM)

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen
cargo install wasm-bindgen-cli

# Build for WASM
cargo build --release --target wasm32-unknown-unknown

# Generate JavaScript bindings
wasm-bindgen --out-dir examples/showcase/pkg --target web target/wasm32-unknown-unknown/release/showcase.wasm

# Serve the files
# You need a web server to serve the files due to CORS restrictions
# Recommended: use the included Rust static file server:
cargo run --bin server

# Then open http://localhost:8080/index.html in your browser
```

## Project Structure

- `main.rs`: Main showcase application
- `index.html`: HTML page for WASM version
- `pkg/`: Generated WASM and JavaScript files
- `Cargo.toml`: Dependencies and build configuration

## Components Demonstrated

### Basic Components
- **Text**: Different text styles (title, headline, body)
- **Button**: Interactive buttons with click handlers
- **Column/Row**: Layout containers
- **Surface**: Root container with background

### Material UI Components
- (Coming soon - currently commented out due to integration issues)

## Development

The showcase uses BECOMPOSE's declarative composable pattern:

```rust
fn ShowcaseApp() {
    Surface(
        Modifiers::new().then(BackgroundModifier::new(Color::srgb(0.05, 0.05, 0.08))),
        || {
            Column(
                Modifiers::new()
                    .fill_max_size()
                    .padding(24.0)
                    .vertical_arrangement(VerticalArrangement::Top)
                    .horizontal_alignment(HorizontalAlignment::Start)
                    .row_gap(32.0),
                || {
                    ShowcaseHeader();
                    BasicComponentsSection();
                },
            );
        },
    );
}
```

## Dependencies

- `becompose`: Local path dependency for the UI framework
- `bevy`: Game engine with webgl2 feature for WASM
- `console_error_panic_hook`: Better error messages in WASM
- `wasm-bindgen`: JavaScript bindings for WASM
# Agent Instructions for Blog Project

## Build Commands
- **Build Rust**: `cargo build` - Compile the Rust application
- **Run development**: `cargo run` - Start the Axum web server
- **Build CSS**: `npx @tailwindcss/cli -i ./assets/input.css -o ./static/css/app.css --watch --minify` - Build Tailwind styles

## Test Commands
- **Run all tests**: `cargo test` - Execute Rust unit and integration tests
- **Run single test**: `cargo test test_name` - Run specific test function
- **Run tests with output**: `cargo test -- --nocapture` - Show test output

## Lint and Format Commands
- **Format code**: `cargo fmt` - Format Rust code with rustfmt
- **Lint code**: `cargo clippy` - Run Clippy linter for Rust
- **Check**: `cargo check` - Fast compilation check without building

## Code Style Guidelines

### Rust Conventions
- Use `anyhow` for error handling with `?` operator
- Follow standard Rust naming: `snake_case` for functions/variables, `CamelCase` for types
- Use `tracing` for logging instead of `println!`
- Prefer `serde` with `#[derive(Serialize, Deserialize)]` for data structures
- Use `tokio` for async operations with `#[tokio::main]` attribute

### Project Structure
- Keep main application logic in `src/main.rs`
- Use standard Rust module structure for larger features
- Separate concerns: handlers, models, templates

### Dependencies
- Axum for web framework
- Tera for HTML templating
- Tailwind CSS for styling (separate Node.js build step)

## Development Workflow
1. Run `cargo check` frequently for fast feedback
2. Use `cargo fmt` and `cargo clippy` before commits
3. Test with `cargo test` to ensure functionality
4. Build CSS separately when making style changes

# Rendevouz
A backend built in rust for the Rendevouz app.
## Building
How to build the Rendevouz App

### Dev
- `cargo build`
- `cargo run`

### Prod
- `cargo build --release`
- `cargo run --release`

## Coding
- Running cargo watch will build after every code change.
`cargo install cargo-watch`
`cargo watch -x check`

## Testing
- Run tests for the rendevouz application.
`cargo test`

### Code Coverage
- `cargo install cargo-tarpaulin`
- `cargo tarpaulin --ignore-tests --out Xml`

## Linting
- You might have to install clippy first.
`rustup component add clippy`
- To run clippy you can use:
`cargo clippy`
- In our CI pipeline we will use:
`cargo clippy -- -D warnings`
- Which will fail if there are any warnings.
`[allow(clippy::lint_name)]` Can be used in code blocks to ignore a specific lint warning.
## Code Formatting
- You might have to install fmt using:
`rustup component add rustfmt`
- To run fmt you can use:
`cargo fmt`
- In our pipeline we will use:
`cargo fmt -- --check`

## Security
- Using crates sets us up for supply chain hacks. Lets find a way to mitigate this.
- We can use cargo audit to check for vulnerabilities. `cargo install cargo-audit`
- We can run cargo-audit: `cargo audit`
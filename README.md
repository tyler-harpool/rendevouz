# Rendevouz
A backend built in rust for the Rendevouz app.
## Setup Postgres Database
This will install the chainguard container image with Postgres. Change the default variable values in scripts/init_db.sh. 
- `chmod +x scripts/init_db.sh`
- `./scripts/init_db.sh`
- Prevent rebuilding docker and run sqlx database migrations: `SKIP_DOCKER=true ./scripts/init_db.sh`

### Install sqlx client
- Install psql using your package manager of choice.
- `cargo install --version="~0.6" sqlx-cli --no-default-features \
--features rustls,postgres`

- Test to see if it installed correct: `sqlx --help`

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
- `cargo install cargo-watch`
- `cargo watch -x check`

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

## CI/CD
- We will use github actions for our CI/CD pipeline. You can find the respective files in this repo:
`.github/workflows/pipeline.yaml` and `.github/workflows/audit.yaml`

## Inspecting Macros
- Macros main focus is code generation. We can inspect them using this tool. Macros operate at the token level they take in symbols and output new symbols. #[tokio::main]: is an example. 
- `cargo install cargo-expand`
- `cargo expand`
- Relies on nightly compiler and not stable
- `rustup toolchain install nightly --allow-downgrade`

# SSH keys
- Make sure ssh-agent is running: `eval "$(ssh-agent -s)"`
- Add your ssh key to the agent store: `ssh-add ~/.ssh/id_ed25519`

## Useful Links
[How to setup two github accounts](https://gist.github.com/rahularity/86da20fe3858e6b311de068201d279e3)
use newsletter::{configuration, startup};

// Tip: install `cargo expand` to inspect the boilerplate code generated by the `tokio::main`
// procedural macro:
// 1. `rustup toolchain install nightly --allow-downgrade` (if not yet installed)
// 2. `cargo +nightly expand --bin newsletter`
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = configuration::parse()
        .expect("Failed to parse the application config")
        .server
        .listener()?;

    startup::run_server(listener)?.await
}

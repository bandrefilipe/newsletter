use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is
/// indeed quite complex.
/// We need to explicitly call out that the returned subscriber is
/// `Send` and `Sync` to make it possible to pass it to [init_subscriber]
/// later on.
pub fn get_subscriber<S: AsRef<str>>(name: String, directives: S) -> impl Subscriber + Send + Sync {
    // Layer that filters spans and events based on a set of filter directives
    let env_filter = EnvFilter::try_from_default_env() //
        .unwrap_or_else(|_| EnvFilter::new(directives));
    // Layer that formats information using the Bunyan format
    // It relies on the JsonStorageLayer to get access to the fields attached to each span
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init() //
        .expect("Failed to set the logger");
    tracing::subscriber::set_global_default(subscriber) //
        .expect("Failed to set the tracing subscriber");
}

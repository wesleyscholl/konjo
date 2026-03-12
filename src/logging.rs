use tracing_subscriber::{fmt, EnvFilter};

pub fn init(default_filter: &str) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_filter.to_owned()));

    fmt().with_env_filter(filter).with_target(false).init();
}
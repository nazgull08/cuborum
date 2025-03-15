use tracing::{info};
use tracing_subscriber::{fmt, EnvFilter};

mod app;
mod renderer;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Cuborum MVP started.");

    pollster::block_on(app::run());
}

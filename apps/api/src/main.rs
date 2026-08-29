mod router;
mod shutdown;
mod state;

use observability::init_tracing;
use std::net::SocketAddr;
use tracing::info;

use crate::{router::build_router, shutdown::shutdown_signal, state::AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let config = app_config::Config::load()?;
    let state = AppState::new(config);

    let app = build_router(state.clone());

    let address = SocketAddr::new(state.config.server.host.parse()?, state.config.server.port);

    let listener = tokio::net::TcpListener::bind(address).await?;

    info!(address = %address, "t1diaries api started");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

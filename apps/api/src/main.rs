mod graphql_handlers;
mod middleware;
mod router;
mod shutdown;
mod state;

use graphql::build_schema;
use observability::init_tracing;
use std::net::SocketAddr;
use tracing::info;

use crate::{router::build_router, shutdown::shutdown_signal, state::AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let app_config = app_config::AppConfig::load()?;
    let schema = build_schema();
    let state = AppState::new(app_config, schema);

    let app = build_router(state.clone());

    let address = SocketAddr::new(state.config.server.host.parse()?, state.config.server.port);

    let listener = tokio::net::TcpListener::bind(address).await?;

    info!(address = %address, "t1diaries api started");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

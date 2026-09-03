pub fn init_tracing() {
    // TODO: (maaahad) subscribe other dependencies/crates's parsing level
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "api=info,tower_http=info".into()),
        )
        .init();
}

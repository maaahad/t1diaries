pub fn init_tracing() {
    tracing_subscriber::fmt().init();
    // tracing_subscriber::fmt()
    //     .with_env_filter(
    //         std::env::var("RUST_LOG").unwrap_or_else(|_| "t1diaries=info, tower_http=info".into()),
    //     )
    //     .json()
    //     .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = app_config::AppConfig::load()?;
    // TODO: config validation and fails instantly if validation failed
    config.validate()?;

    println!("{:?}", config);

    Ok(())
}

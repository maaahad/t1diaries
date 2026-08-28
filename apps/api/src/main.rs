mod router;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = app_config::Config::load()?;

    println!("{:?}", config);

    Ok(())
}

use anyhow::Ok;
use remotex::domain::settings::AppSettings;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cfg = AppSettings::load_default_config()?;

    println!("Loaded settings: {:?}", cfg);

    remotex::web::server::serve_web_server(cfg.clone()).await?;

    Ok(())
}

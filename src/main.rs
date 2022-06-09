use actix_web::{App};
use remotex::domain::settings::AppSettings;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cfg: AppSettings = AppSettings::load_config()?;

    println!("Loaded settings: {:?}", cfg);

    Ok(())
}

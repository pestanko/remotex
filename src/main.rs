use std::path::Path;

use anyhow::Ok;
use remotex::domain::settings::AppSettings;
use structopt::StructOpt;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = CliApp::from_args();

    let cfg = match &args.root {
        Some(root) => AppSettings::load_config(Path::new(root))?,
        None => AppSettings::load_default_config()?,
    };

    println!("Loaded settings: {:?}", cfg);

    match args.sub {
        SubCmd::Serve(serve) => {
            let mut settings = cfg.clone();
            if let Some(ref addr) = serve.addr {
                settings.web.addr = addr.clone();
            }
            remotex::web::server::serve_web_server(settings).await?;
        }
    };

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "remotex")]
struct CliApp {
    #[structopt(subcommand)]
    pub sub: SubCmd,
    #[structopt(short, long)]
    pub root: Option<String>,
}

#[derive(Debug, StructOpt)]
pub enum SubCmd {
    #[structopt(name = "serve")]
    Serve(Serve),
}

#[derive(Debug, StructOpt)]
pub struct Serve {
    pub addr: Option<String>,
}

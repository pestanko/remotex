use anyhow::Ok;
use remotex::domain::settings::AppSettings;
use structopt::StructOpt;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = CliApp::from_args();

    let cfg = AppSettings::load_default_config()?;

    println!("Loaded settings: {:?}", cfg);

    match args.sub {
        SubCmd::Serve(_serve) => {
            remotex::web::server::serve_web_server(cfg.clone()).await?;
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
    pub bar: Option<String>,
}

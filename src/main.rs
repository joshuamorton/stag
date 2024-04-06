use clap::{Args, Parser, Subcommand};

mod init;
mod settings;

#[derive(Parser)]
#[command(version, about, long_about=None)]
#[command(about = "Lets you tag and query music stored in spotify.")]
struct Cli {
    #[arg(short, long, default_value = "stag.toml")]
    config_file: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init(InitArgs),
}

#[derive(Args)]
#[command(about = "Initializes the spotify playlist we use")]
struct InitArgs {}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let cfg = settings::StagConfig::from_file(&cli.config_file).unwrap_or_else(|_| {
        panic!(
            "the config file `{}` should contain a valid toml config with settings.",
            cli.config_file
        )
    });
    match &cli.command {
        Commands::Init(_) => init::init(cfg).await,
    }
}

use clap::{Args, Parser, Subcommand};

mod connection;
mod db;
mod init;
mod query;
mod settings;
mod tag;

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
    Tag(TagArgs),
    Query(QueryArgs),
}

#[derive(Args)]
#[command(about = "Initializes the spotify playlist we use")]
struct InitArgs {}

#[derive(Args)]
#[command(about = "Add tags to a song")]
struct TagArgs {
    #[arg(short, long, default_value_t = true)]
    /// Default case: add tags to the currently playing song.
    current: bool,
    #[arg(short, long)]
    /// Song title to add tags to, overrides `--current`
    song: Option<String>,
    /// The tags to add.
    tags: Vec<String>,
}

#[derive(Args)]
#[command(about = "Queries songs with matching tags")]
struct QueryArgs {
    /// The tag to query matching songs with
    tag: String,
}

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
        Commands::Init(_) => init::init(&cfg).await,
        Commands::Tag(args) => tag::tag(args.current, &args.song, &args.tags, &cfg)
            .await
            .expect("Should have worked :P"),
        Commands::Query(args) => query::query(&args.tag, &cfg).expect("Should have worked :P"),
    }
}

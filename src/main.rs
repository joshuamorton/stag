use clap::Parser;

mod settings;

#[derive(Parser)]
#[command(version, about, long_about=None)]
#[command(about = "Lets you tag and query music stored in spotify.")]
struct Cli {
    #[arg(short, long, default_value = "stag.toml")]
    config_file: String,
}

fn main() {
    let cli = Cli::parse();
    let cfg = settings::StagConfig::from_file(&cli.config_file);
    match cfg {
        Err(e) => println!("{}", e),
        Ok(x) => println!("{:?}", x),
    }
}

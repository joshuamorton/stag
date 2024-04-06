use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
#[command(about="Lets you tag and query music stored in spotify.")]
struct Cli {
    #[arg(short, long, default_value = "stag.toml")]
    config_file: String,
}


fn main() {
    let cli = Cli::parse();
    println!("Hello, world!");
}

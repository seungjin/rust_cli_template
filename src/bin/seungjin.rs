use clap::{Parser, Subcommand};
use std::ffi::OsString;

#[derive(Parser, Debug)]
#[clap(
    author = "Author Name",
    version,
    about = "A Very simple Package Hunter"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(external_subcommand)]
    External(Vec<OsString>),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::External(args) => {
            seungjin_add::run(&args[0..]);
        }
    }
}

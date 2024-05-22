use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::{ffi::OsString, ops::Deref};

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

    let mut mods = HashMap::new();
    mods.insert("add", seungjin_add::run);

    match cli.command {
        Commands::External(args) => {
            let cname = &*args[0].to_str().unwrap();

            if !mods.contains_key(cname) {
                println!("Command not found!")
            } else {
                let cmd = mods.get(cname).unwrap();
                cmd(&args);
            }
        }
    }
}

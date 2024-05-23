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

//struct ExteralCommand {
//    command: String,
//    function: Fn,
//}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // let mut mods = HashMap::new();
    // mods.insert("add", seungjin_add::run);
    // mods.insert("concat", seungjin_concat::run);

    match cli.command {
        Commands::External(args) => {
            let cname = &*args[0].to_str().unwrap();

            match cname {
                "add" => seungjin_add::run(&args),
                "concat" => seungjin_concat::run(&args),
                _ => {
                    println!("Command `{}` not found", cname)
                }
            }

            // if !mods.contains_key(cname) {
            //     println!("Command `{}` not found!", cname);
            // } else {
            //     let cmd = mods.get(cname).unwrap();
            //     cmd(&args);
            // }
        }
    }
}

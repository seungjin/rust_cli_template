use seungjin_concat::concat;
use seungjin_concat::Cli;

use clap::Parser;
use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    let cli = Cli::parse();
    seungjin_concat::inner_run(cli);
}

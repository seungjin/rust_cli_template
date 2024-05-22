use seungjin_add::add;
use seungjin_add::Cli;

use clap::Parser;
use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    let cli = Cli::parse();
    seungjin_add::inner_run(cli);
}

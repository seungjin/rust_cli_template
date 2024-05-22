use clap::builder::OsStr;
use clap::Parser;
use clap::{arg, command, value_parser, ArgAction, Command};
use std::ffi::OsString;

#[derive(Parser)]
#[command(name = "Add")]
#[command(version = "1.0")]
#[command(about = "Add!", long_about = None)]
pub struct Cli {
    pub x: i64,
    pub y: i64,
}

pub fn add(x: i64, y: i64) -> i64 {
    x + y
}

pub fn run(args: &[OsString]) {
    let cli = Cli::parse_from(args);
    inner_run(cli);
}

pub fn inner_run(c: Cli) {
    let r = add(c.x, c.y);
    println!("{} + {} = {}", c.x, c.y, r);
}

// allow_external_subcommands
// https://docs.rs/clap/latest/clap/struct.Command.html#method.allow_external_subcommands

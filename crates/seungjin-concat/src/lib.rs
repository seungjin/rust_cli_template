use clap::command;
use clap::Parser;
use std::ffi::OsString;

#[derive(Parser)]
#[command(name = "Concat")]
#[command(version = "1.0")]
#[command(about = "concat", long_about = None)]
pub struct Cli {
    pub x: String,
    pub y: String,
}

pub trait Foo {
    fn bar(args: &[OsString]);
}

pub fn run(args: &[OsString]) {
    let cli = Cli::parse_from(args);
    inner_run(cli);
}

pub fn inner_run(c: Cli) {
    let r = concat(&c.x, &c.y);
    println!("{} + {} = {}", c.x, c.y, r);
}

pub fn concat(x: &String, y: &String) -> String {
    format!("{}{}", x, y)
}

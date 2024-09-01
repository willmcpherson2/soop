use soop::{deep_to_exp, eval, parse, print, Cli, Env};

use clap::Parser;
use std::fs;

fn main() {
    let cli = Cli::parse();

    let text = if cli.expression {
        cli.target
    } else {
        match fs::read_to_string(cli.target) {
            Ok(text) => text,
            Err(e) => return eprintln!("Error reading file: {}", e),
        }
    };

    let exp = parse(&text);
    let deep = eval(Env::new(), exp);
    let exp = deep_to_exp(deep);
    println!("{}", print(exp));
}

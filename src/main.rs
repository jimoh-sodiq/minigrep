use minigrep;
use std::{env::args, process};

fn main() {

    let arguments: Vec<String> = args().collect();
    for argument in arguments.iter() {
        println!("{}", argument);
    }

    let config = minigrep::Config::new(arguments).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        process::exit(1);
    });

    minigrep::run(&config).unwrap_or_else(|err| {
        println!("Err: {err}");
        process::exit(1);
    });
}

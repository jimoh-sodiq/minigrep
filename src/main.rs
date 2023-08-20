use minigrep;
use std::{env::args, process};

fn main() {
    // todo
    // [] -  refactor file handling
    // [] -  enhance the file error handling
    // [] -  make sure everthing still works
    // [] -  refactor file handling

    let arguments: Vec<String> = args().collect();
    for argument in arguments.iter() {
        println!("{}", argument);
    }

    let config = minigrep::Config::new(arguments).unwrap_or_else(|err| {
        println!("Error: {err}");
        process::exit(1);
    });

    minigrep::run(&config).unwrap_or_else(|err| {
        println!("Err: {err}");
        process::exit(1);
    });
}

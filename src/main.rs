use std::env;

mod help;
mod colors;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help::help();
        std::process::exit(0)
    }

    match args[1].as_str() {
        "help" => help::help(),
        _ => help::help()
    }
}

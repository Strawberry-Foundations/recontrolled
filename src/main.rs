use std::env;

mod colors;
mod basics {
    pub mod help;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        basics::help::print_help();
        std::process::exit(0)
    }

    match args[1].as_str() {
        "help" => basics::help::print_help(),
        _ => basics::help::print_help()
    }
}

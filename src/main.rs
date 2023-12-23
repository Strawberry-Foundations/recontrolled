use std::env;

mod vars {
    pub mod colors;
    pub mod constants;
}
mod basics {
    pub mod help;
    pub mod about;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        basics::help::print_help();
        std::process::exit(0)
    }

    match args[1].as_str() {
        "help" => basics::help::print_help(),
        "about" => basics::about::print_about(),
        _ => basics::help::print_help()
    }
}

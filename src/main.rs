use std::env;
use crate::vars::colors::RED;

mod vars {
    pub mod colors;
    pub mod constants;
    pub mod modules;
}
mod basics {
    pub mod help;
    pub mod about;
    pub mod version;
    pub mod status;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if env::consts::OS != "linux" {
        eprintln!("{}Platform '{}' is not supported, only Linux is supported.", RED, env::consts::OS);
        std::process::exit(1)
    }

    if args.len() < 2 {
        basics::help::print_help();
        std::process::exit(0)
    }

    match args[1].as_str() {
        "help" => basics::help::print_help(),
        "about" => basics::about::print_about(),
        "version" | "ver" => basics::version::print_version(),
        "status" => basics::status::print_status(),
        _ => basics::help::print_help()
    }
}

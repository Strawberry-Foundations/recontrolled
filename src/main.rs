use std::env;
use crate::vars::colors::RED;

use crate::vars::modules::{LED, Status};

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
mod led {
    pub mod basic_handler;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if !cfg!(target_os = "linux") {
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
        "ar" => led::basic_handler::set_status(LED::PWR, Status::ON),
        "dr" => led::basic_handler::set_status(LED::PWR, Status::OFF),
        "ag" => led::basic_handler::set_status(LED::ACT, Status::ON),
        "dg" => led::basic_handler::set_status(LED::ACT, Status::OFF),
        _ => basics::help::print_help()
    }
}

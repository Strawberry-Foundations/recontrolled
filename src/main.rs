use std::env;

use crate::vars::modules::Status;

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
mod pwr {
    pub mod basic_handler;
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
        "version" | "ver" => basics::version::print_version(),
        "status" => basics::status::print_status(),
        "ar" => pwr::basic_handler::set_status(Status::ON),
        "dr" => pwr::basic_handler::set_status(Status::OFF),
        _ => basics::help::print_help()
    }
}

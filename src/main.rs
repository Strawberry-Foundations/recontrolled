use std::env;
use crate::vars::colors::{BOLD, RED, C_RESET};

use crate::vars::modules::{Led, Status};

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
    pub mod blink;
    pub mod blink_sync;
    pub mod blink_switch;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if !cfg!(target_os = "linux") {
        panic!("{BOLD}{RED}Platform '{}' is not supported! Only Linux is supported!{C_RESET}", env::consts::OS);
    }

    if args.len() < 2 {
        basics::help::print_help();
        std::process::exit(0)
    }

    match args[1].as_str() {
        // BASIC
        "help" => basics::help::print_help(),
        "about" => basics::about::print_about(),
        "version" | "ver" => basics::version::print_version(),
        "status" => basics::status::print_status(),

        // ACTIVATE / DEACTIVATE
        "ar" => led::basic_handler::set_status(Led::Pwr, Status::On),
        "dr" => led::basic_handler::set_status(Led::Pwr, Status::Off),
        "ag" => led::basic_handler::set_status(Led::Act, Status::On),
        "dg" => led::basic_handler::set_status(Led::Act, Status::Off),

        // BLINK (RED)
        "br" => led::blink::blink(Led::Pwr, 1000),
        "brf" => led::blink::blink(Led::Pwr, 500),
        "brs" => led::blink::blink(Led::Pwr, 2000),
        "brff" => led::blink::blink(Led::Pwr, 250),

        // BLINK (GREEN)
        "bg" => led::blink::blink(Led::Act, 1000),
        "bgf" => led::blink::blink(Led::Act, 500),
        "bgs" => led::blink::blink(Led::Act, 2000),
        "bgff" => led::blink::blink(Led::Act, 250),

        // BLINK (CUSTOM)
        "brc" => {
            if args.len() < 3 {
                eprintln!("{RED}{BOLD}Invalid argument for blink delay{C_RESET}");
                basics::help::print_help();
                std::process::exit(0)
            }

            led::blink::blink(Led::Pwr, args[2].parse::<u64>().unwrap_or_else(|_| {
                eprintln!("{RED}{BOLD}Invalid type for blink delay{C_RESET}");
                std::process::exit(1)
            }))
        },
        "bgc" => {
            if args.len() < 3 {
                eprintln!("{RED}{BOLD}Invalid argument for blink delay{C_RESET}");
                basics::help::print_help();
                std::process::exit(0)
            }

            led::blink::blink(Led::Act, args[2].parse::<u64>().unwrap_or_else(|_| {
                eprintln!("{RED}{BOLD}Invalid type for blink delay{C_RESET}");
                std::process::exit(1)
            }))
        },

        // SYNC MODE
        "lb" => led::blink_sync::blink_sync(1000),
        "lbs" => led::blink_switch::blink_switch(1000),

        // SYNC MODE (CUSTOM)
        "lbc" => {
            if args.len() < 3 {
                eprintln!("{RED}{BOLD}Invalid argument for blink delay{C_RESET}");
                basics::help::print_help();
                std::process::exit(0)
            }

            led::blink_sync::blink_sync(args[2].parse::<u64>().unwrap_or_else(|_| {
                eprintln!("{RED}{BOLD}Invalid type for blink delay{C_RESET}");
                std::process::exit(1)
            }))
        },
        "lbsc" => {
            if args.len() < 3 {
                eprintln!("{RED}{BOLD}Invalid argument for blink delay{C_RESET}");
                basics::help::print_help();
                std::process::exit(0)
            }

            led::blink_switch::blink_switch(args[2].parse::<u64>().unwrap_or_else(|_| {
                eprintln!("{RED}{BOLD}Invalid type for blink delay{C_RESET}");
                std::process::exit(1)
            }))
        },

        _ => basics::help::print_help()
    }
}

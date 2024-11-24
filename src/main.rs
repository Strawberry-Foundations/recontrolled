use std::env;
use crate::utils::model::{get_raspberry_pi_model, is_supported_board};
use crate::constants::colors::{BOLD, RED, C_RESET};
use crate::constants::modules::{Led, Status};

mod basics;
mod led;
mod constants;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if !cfg!(target_os = "linux") {
        panic_err!("{BOLD}{RED}Platform '{}' is not supported! Only Linux is supported!{C_RESET}", env::consts::OS);
    }

    if let Some(model) = get_raspberry_pi_model() {
        if !is_supported_board(&model) {
            panic_err!("{BOLD}{RED}This Raspberry Pi Model ({model}) is not supported!{C_RESET}");
        }
    } else {
        panic_err!("{BOLD}{RED}Could not detect Raspberry Pi Model!{C_RESET}");
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

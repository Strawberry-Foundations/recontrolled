use std::env;
use crate::vars::colors::{BOLD, RED, C_RESET};

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
    pub mod blink;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if !cfg!(target_os = "linux") {
        panic!("{BOLD}{RED}Platform '{}' is not supported! Only Linux is supported!{C_RESET}", std::env::consts::OS);
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
        "ar" => led::basic_handler::set_status(LED::PWR, Status::ON),
        "dr" => led::basic_handler::set_status(LED::PWR, Status::OFF),
        "ag" => led::basic_handler::set_status(LED::ACT, Status::ON),
        "dg" => led::basic_handler::set_status(LED::ACT, Status::OFF),

        // BLINK (RED)
        "br" => led::blink::blink(LED::PWR, 1000),
        "brf" => led::blink::blink(LED::PWR, 500),
        "brs" => led::blink::blink(LED::PWR, 2000),
        "brff" => led::blink::blink(LED::PWR, 250),

        // BLINK (GREEN)
        "bg" => led::blink::blink(LED::ACT, 1000),
        "bgf" => led::blink::blink(LED::ACT, 500),
        "bgs" => led::blink::blink(LED::ACT, 2000),
        "bgff" => led::blink::blink(LED::ACT, 250),
        _ => basics::help::print_help()
    }
}

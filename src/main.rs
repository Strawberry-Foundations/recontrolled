use crate::constants::colors::{BOLD, C_RESET, RED};
use crate::core::led::{Led, Status};
use crate::core::model::{RaspberryPi, RaspberryPiModel};
use crate::core::rp02w::RaspberryPiZero2W;
use crate::core::rp4::RaspberryPi4B;
use crate::core::rp5::RaspberryPi5B;
use crate::utils::model::get_raspberry_pi_model;

use std::env;
use std::env::consts::OS;

mod basics;
mod constants;
mod core;
mod led;
mod utils;

const DEFAULT_BLINK_DELAY: u64 = 1000;
const FAST_BLINK_DELAY: u64 = 500;
const SLOW_BLINK_DELAY: u64 = 2000;
const SUPER_FAST_DELAY: u64 = 250;

fn handle_custom_blink(args: &[String], led: Led) {
    if args.len() < 3 {
        eprintln!("{RED}{BOLD}Invalid argument for blink delay{C_RESET}");
        basics::help::print_help();
        std::process::exit(1);
    }

    let delay = args[2].parse::<u64>().unwrap_or_else(|_| {
        eprintln!("{RED}{BOLD}Invalid type for blink delay{C_RESET}");
        std::process::exit(1);
    });

    led::blink::blink(led, delay);
}

fn handle_sync_mode(args: &[String], sync_fn: fn(u64)) {
    if args.len() < 3 {
        sync_fn(DEFAULT_BLINK_DELAY);
        return;
    }

    let delay = args[2].parse::<u64>().unwrap_or_else(|_| {
        eprintln!("{RED}{BOLD}Invalid type for blink delay{C_RESET}");
        std::process::exit(1);
    });

    sync_fn(delay);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if !cfg!(target_os = "linux") {
        panic_err!(
            "{BOLD}{RED}Platform '{OS}' is not supported! Only Linux is supported!{C_RESET}"
        );
    }

    let model: Box<dyn RaspberryPi> = match get_raspberry_pi_model()
        .and_then(|model| RaspberryPiModel::from_raw(&model))
        .unwrap_or_else(|| {
            if env::args().collect::<Vec<String>>().contains(&String::from("--force")) {
                return RaspberryPiModel::Pi4B;
            }

            eprintln!("{RED}{BOLD}This Raspberry Pi Model is not supported!{C_RESET}");
            std::process::exit(1);
        }) {
        RaspberryPiModel::Pi4B => Box::new(RaspberryPi4B::new()),
        RaspberryPiModel::Pi5B => Box::new(RaspberryPi5B::new()),
        RaspberryPiModel::PiZero2W => Box::new(RaspberryPiZero2W::new()),
    };

    if args.len() < 2 {
        basics::help::print_help();
        std::process::exit(0);
    }

    match args[1].as_str() {
        // Basic Commands
        "about" => basics::about::print_about(model),
        "version" | "ver" => basics::version::print_version(),
        "status" => basics::status::print_status(model),

        // LED Control
        "ar" => led::basic_handler::set_status(Led::Pwr, Status::On),
        "dr" => led::basic_handler::set_status(Led::Pwr, Status::Off),
        "ag" => led::basic_handler::set_status(Led::Act, Status::On),
        "dg" => led::basic_handler::set_status(Led::Act, Status::Off),

        // Blink modes for Power LED
        "br" => led::blink::blink(Led::Pwr, DEFAULT_BLINK_DELAY),
        "brf" => led::blink::blink(Led::Pwr, FAST_BLINK_DELAY),
        "brs" => led::blink::blink(Led::Pwr, SLOW_BLINK_DELAY),
        "brff" => led::blink::blink(Led::Pwr, SUPER_FAST_DELAY),
        "brc" => handle_custom_blink(&args, Led::Pwr),

        // Blink modes for Activity LED
        "bg" => led::blink::blink(Led::Act, DEFAULT_BLINK_DELAY),
        "bgf" => led::blink::blink(Led::Act, FAST_BLINK_DELAY),
        "bgs" => led::blink::blink(Led::Act, SLOW_BLINK_DELAY),
        "bgff" => led::blink::blink(Led::Act, SUPER_FAST_DELAY),
        "bgc" => handle_custom_blink(&args, Led::Act),

        // Sync modes
        "lb" => led::blink_sync::blink_sync(DEFAULT_BLINK_DELAY),
        "lbs" => led::blink_switch::blink_switch(DEFAULT_BLINK_DELAY),
        "lbc" => handle_sync_mode(&args, led::blink_sync::blink_sync),
        "lbsc" => handle_sync_mode(&args, led::blink_switch::blink_switch),
        _ => basics::help::print_help(),
    }
}

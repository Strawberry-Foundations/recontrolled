#![warn(clippy::pedantic, clippy::nursery)]

use crate::constants::colors::{BOLD, C_RESET, RED};
use crate::core::led::{Led, Status};
use crate::core::hardware::{RaspberryPi, RaspberryPiModel, get_raspberry_pi_model};
use crate::core::rp02w::RaspberryPiZero2W;
use crate::core::rp4::RaspberryPi4B;
use crate::core::rp5::RaspberryPi5B;
use crate::led::LEDController;

use std::env;

mod basics;
mod constants;
mod core;
mod led;
mod utils;

const DEFAULT_BLINK_DELAY: u64 = 1000;
const FAST_BLINK_DELAY: u64 = 500;
const SLOW_BLINK_DELAY: u64 = 2000;
const SUPER_FAST_DELAY: u64 = 250;

fn main() {
    let args: Vec<String> = env::args().collect();

    if !cfg!(target_os = "linux") {
        panic_err!("{BOLD}{RED}Recontrolled does only support Linux / Raspberry Pi OS!{C_RESET}");
    }

    let hardware: Box<dyn RaspberryPi> = match get_raspberry_pi_model()
        .and_then(|model| RaspberryPiModel::from_raw(&model))
        .unwrap_or_else(|| {
            if env::args().any(|x| x == "--force") {
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

    let controller = LEDController::new(hardware);

    match args[1].as_str() {
        // Basic Commands
        "about" => basics::about::about(controller.hardware),
        "version" | "ver" => basics::version::version(),
        "status" => basics::status::status(controller.hardware),

        // LED Control for Power LED
        "pwr" => match args[2].as_str() {
            "on" => controller.set_status(Led::Pwr, Status::On),
            "off" => controller.set_status(Led::Pwr, Status::Off),
            "blink" => controller.blink(Led::Pwr, DEFAULT_BLINK_DELAY),
            "blink_fast" => controller.blink(Led::Pwr, FAST_BLINK_DELAY),
            "blink_slow" => controller.blink(Led::Pwr, SLOW_BLINK_DELAY),
            "blink_super_fast" => controller.blink(Led::Pwr, SUPER_FAST_DELAY),
            "blink_custom" => {
                if args.len() < 4 {
                    eprintln!("{RED}{BOLD}Invalid argument for blink delay{C_RESET}");
                    basics::help::print_help();
                    std::process::exit(1);
                }
                let delay = args[3].parse::<u64>().unwrap_or_else(|_| {
                    eprintln!("{RED}{BOLD}Invalid type for blink delay{C_RESET}");
                    std::process::exit(1);
                });
                controller.blink(Led::Pwr, delay);
            }
            _ => basics::help::print_help(),
        },

        "act" => match args[2].as_str() {
            "on" => controller.set_status(Led::Act, Status::On),
            "off" => controller.set_status(Led::Act, Status::Off),
            "blink" => controller.blink(Led::Act, DEFAULT_BLINK_DELAY),
            "blink_fast" => controller.blink(Led::Act, FAST_BLINK_DELAY),
            "blink_slow" => controller.blink(Led::Act, SLOW_BLINK_DELAY),
            "blink_super_fast" => controller.blink(Led::Act, SUPER_FAST_DELAY),
            "blink_custom" => {
                if args.len() < 4 {
                    eprintln!("{RED}{BOLD}Invalid argument for blink delay{C_RESET}");
                    basics::help::print_help();
                    std::process::exit(1);
                }
                let delay = args[3].parse::<u64>().unwrap_or_else(|_| {
                    eprintln!("{RED}{BOLD}Invalid type for blink delay{C_RESET}");
                    std::process::exit(1);
                });
                controller.blink(Led::Act, delay);
            }
            _ => basics::help::print_help(),
        },

        // Sync modes
        "blink_sync" => controller.blink_sync(DEFAULT_BLINK_DELAY),
        "blink_switch" => controller.blink_switch(DEFAULT_BLINK_DELAY),
        "blink_sync_custom" => {
            if args.len() < 3 {
                eprintln!("{RED}{BOLD}Invalid argument for blink delay{C_RESET}");
                basics::help::print_help();
                std::process::exit(1);
            }
            let delay = args[2].parse::<u64>().unwrap_or_else(|_| {
                eprintln!("{RED}{BOLD}Invalid type for blink delay{C_RESET}");
                std::process::exit(1);
            });
            controller.blink_sync(delay);
        }
        "blink_switch_custom" => {
            if args.len() < 3 {
                eprintln!("{RED}{BOLD}Invalid argument for blink delay{C_RESET}");
                basics::help::print_help();
                std::process::exit(1);
            }
            let delay = args[2].parse::<u64>().unwrap_or_else(|_| {
                eprintln!("{RED}{BOLD}Invalid type for blink delay{C_RESET}");
                std::process::exit(1);
            });
            controller.blink_switch(delay);
        }
        _ => basics::help::print_help(),
    }
}

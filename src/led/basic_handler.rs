use std::fs::File;
use std::io::Write;
use crate::constants::colors::{BOLD, GREEN, RED, WHITE, C_RESET, LIGHT_GREEN};
use crate::constants::constants::{FILE_ACT_LED, FILE_PWR_LED};
use crate::constants::modules::{Led, Status};

pub fn set_status(led: Led, status: Status) {
    let led_file_path = match led {
        Led::Pwr => FILE_PWR_LED,
        Led::Act => FILE_ACT_LED
    };

    let led_string = match led {
        Led::Pwr => format!("{}Power-LED", RED),
        Led::Act => format!("{}Activity-LED", LIGHT_GREEN)
    };

    let mut led_file = match led {
        Led::Pwr => File::create(FILE_PWR_LED).unwrap_or_else(|error| {
            eprintln!("{}{}Error while opening {}: {}{}", BOLD, RED, FILE_PWR_LED, error, C_RESET);
            std::process::exit(1);
        }),
        Led::Act => File::create(FILE_ACT_LED).unwrap_or_else(|error| {
            eprintln!("{}{}Error while opening {}: {}{}", BOLD, RED, FILE_ACT_LED, error, C_RESET);
            std::process::exit(1);
        })
    };

    match status {
        Status::On => {
            led_file.write_all("1".as_bytes()).unwrap_or_else(|error| {
                eprintln!("{}{}Error while writing to {}: {}{}", BOLD, RED, led_file_path, error, C_RESET);
                std::process::exit(1);
            });
            println!("{BOLD}{led_string} {WHITE}was {GREEN}activated{C_RESET}");
        }
        Status::Off => {
            led_file.write_all("0".as_bytes()).unwrap_or_else(|error| {
                eprintln!("{}{}Error while writing to {}: {}{}", BOLD, RED, led_file_path, error, C_RESET);
                std::process::exit(1);
            });
            println!("{BOLD}{led_string} {WHITE}was {RED}deactivated{C_RESET}");
        }
    }
}
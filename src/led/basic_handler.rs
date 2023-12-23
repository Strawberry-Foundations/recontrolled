use std::fs::File;
use std::io::Write;
use crate::vars::colors::{BOLD, GREEN, RED, WHITE, C_RESET, LIGHT_GREEN};
use crate::vars::constants::{FILE_ACT_LED, FILE_PWR_LED};
use crate::vars::modules::{LED, Status};

pub fn set_status(led: LED, status: Status) {
    let led_file_path = match led {
        LED::PWR => FILE_PWR_LED,
        LED::ACT => FILE_ACT_LED
    };

    let led_string = match led {
        LED::PWR => format!("{}Power-LED", RED),
        LED::ACT => format!("{}Activity-LED", LIGHT_GREEN)
    };

    let mut led_file = match led {
        LED::PWR => File::create(FILE_PWR_LED).unwrap_or_else(|error| {
            eprintln!("{}{}Error while opening {}: {}{}", BOLD, RED, FILE_PWR_LED, error, C_RESET);
            std::process::exit(1);
        }),
        LED::ACT => File::create(FILE_ACT_LED).unwrap_or_else(|error| {
            eprintln!("{}{}Error while opening {}: {}{}", BOLD, RED, FILE_ACT_LED, error, C_RESET);
            std::process::exit(1);
        })
    };

    match status {
        Status::ON => {
            led_file.write_all("1".as_bytes()).unwrap_or_else(|error| {
                eprintln!("{}{}Error while writing to {}: {}{}", BOLD, RED, led_file_path, error, C_RESET);
                std::process::exit(1);
            });
            println!("{BOLD}{led_string} {WHITE}was {GREEN}activated{C_RESET}");
        },
        Status::OFF => {
            led_file.write_all("0".as_bytes()).unwrap_or_else(|error| {
                eprintln!("{}{}Error while writing to {}: {}{}", BOLD, RED, led_file_path, error, C_RESET);
                std::process::exit(1);
            });
            println!("{BOLD}{led_string} {WHITE}was {RED}deactivated{C_RESET}");
        },
    }
}
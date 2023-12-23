use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use crate::vars::colors::{BOLD, GREEN, RED, WHITE, C_RESET, LIGHT_GREEN};
use crate::vars::constants::{FILE_ACT_LED, FILE_PWR_LED};
use crate::vars::modules::{LED};

pub fn blink(led: LED, delay: u64) {
    let blink_mode = match delay {
        2000 => "Slow ",
        1000 => "",
        500 => "Fast ",
        250 => "Super Fast ",
        _ => "Custom "
    };

    let led_file_path = match led {
        LED::PWR => FILE_PWR_LED,
        LED::ACT => FILE_ACT_LED
    };

    let led_string = match led {
        LED::PWR => format!("{}Power-LED", RED),
        LED::ACT => format!("{}Activity-LED", LIGHT_GREEN)
    };

    println!("{BOLD}{WHITE}{blink_mode}Blink mode for {led_string}{WHITE} was {GREEN}activated{C_RESET}");

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

    loop {
        thread::sleep(Duration::from_millis(delay));
        led_file.write_all("1".as_bytes()).unwrap_or_else(|error| {
            eprintln!("{}{}Error while writing to {}: {}{}", BOLD, RED, led_file_path, error, C_RESET);
            std::process::exit(1);
        });

        thread::sleep(Duration::from_millis(delay));
        led_file.write_all("0".as_bytes()).unwrap_or_else(|error| {
            eprintln!("{}{}Error while writing to {}: {}{}", BOLD, RED, led_file_path, error, C_RESET);
            std::process::exit(1);
        });
    }
}
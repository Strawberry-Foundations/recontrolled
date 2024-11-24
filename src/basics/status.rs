use std::fs::File;
use std::io::Read;

use crate::constants::colors::{BOLD, C_RESET, CYAN, GREEN, LIGHT_GREEN, RED, WHITE};
use crate::constants::constants::{RECONTROLLED_STR, FILE_ACT_LED, FILE_PWR_LED};
use crate::constants::modules::Led;

pub fn print_status() {
    println!(
        "         {RECONTROLLED_STR}

          LED-Status
* -------------------------- *
|                            |
| {RED}Power-LED {CYAN}   (led1){C_RESET}    {} |
| {LIGHT_GREEN}Activity-LED {CYAN}(led0){C_RESET}    {} |
|                            |
* -------------------------- *{C_RESET}
        ",
        check_status(Led::Pwr), check_status(Led::Act)
    )
}

fn check_status(led: Led) -> String {
    let led_file_path = match led {
        Led::Pwr => FILE_PWR_LED,
        Led::Act => FILE_ACT_LED
    };

    let mut led_file = match led {
        Led::Pwr => File::open(FILE_PWR_LED).unwrap_or_else(|_| panic!("{}Error while opening {}{}", RED, FILE_PWR_LED, C_RESET)),
        Led::Act => File::open(FILE_ACT_LED).unwrap_or_else(|_| panic!("{}Error while opening {}{}", RED, FILE_ACT_LED, C_RESET)),
    };

    let mut led_status = String::new();

    let fmt_off = format!("{}{}off{}", BOLD, RED, WHITE);
    let fmt_on = format!("{}{}on {}", BOLD, GREEN, WHITE);

    led_file.read_to_string(&mut led_status).unwrap_or_else(|_| panic!("{}Error while reading {}{}", RED, led_file_path, C_RESET));

    match led_status.as_str().trim() {
        "0" => fmt_off,
        "255" | "1" => fmt_on,
        _ => fmt_off
    }
}
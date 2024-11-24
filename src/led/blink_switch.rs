use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use crate::constants::colors::{BOLD, GREEN, RED, WHITE, C_RESET, LIGHT_GREEN};
use crate::constants::constants::{FILE_ACT_LED, FILE_PWR_LED};

pub fn blink_switch(delay: u64) {
    let blink_mode = match delay {
        2000 => "Slow ",
        1000 => "",
        500 => "Fast ",
        250 => "Super Fast ",
        _ => "Custom "
    };

    println!("{BOLD}{WHITE}{blink_mode}Switching Blink mode for {RED}Power-{WHITE} and {LIGHT_GREEN}Activity-LED{WHITE} was {GREEN}activated{C_RESET}");

    let mut pwr_led_file = File::create(FILE_PWR_LED).unwrap_or_else(|error| {
        eprintln!("{}{}Error while opening {}: {}{}", BOLD, RED, FILE_PWR_LED, error, C_RESET);
        std::process::exit(1);
    });

    let mut act_led_file = File::create(FILE_ACT_LED).unwrap_or_else(|error| {
        eprintln!("{}{}Error while opening {}: {}{}", BOLD, RED, FILE_ACT_LED, error, C_RESET);
        std::process::exit(1);
    });


    loop {
        thread::sleep(Duration::from_millis(delay));
        pwr_led_file.write_all("1".as_bytes()).unwrap_or_else(|error| {
            eprintln!("{}{}Error while writing to {}: {}{}", BOLD, RED, FILE_PWR_LED, error, C_RESET);
            std::process::exit(1);
        });
        act_led_file.write_all("0".as_bytes()).unwrap_or_else(|error| {
            eprintln!("{}{}Error while writing to {}: {}{}", BOLD, RED, FILE_ACT_LED, error, C_RESET);
            std::process::exit(1);
        });

        thread::sleep(Duration::from_millis(delay));
        pwr_led_file.write_all("0".as_bytes()).unwrap_or_else(|error| {
            eprintln!("{}{}Error while writing to {}: {}{}", BOLD, RED, FILE_PWR_LED, error, C_RESET);
            std::process::exit(1);
        });
        act_led_file.write_all("1".as_bytes()).unwrap_or_else(|error| {
            eprintln!("{}{}Error while writing to {}: {}{}", BOLD, RED, FILE_ACT_LED, error, C_RESET);
            std::process::exit(1);
        });
    }
}
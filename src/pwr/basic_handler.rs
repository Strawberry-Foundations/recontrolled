use std::fs::File;
use std::io::Write;
use crate::vars::colors::{BOLD, C_RESET, GREEN, RED, WHITE};
use crate::vars::constants::{FILE_PWR_LED};
use crate::vars::modules::{Status};

pub fn set_status(status: Status) {
    let mut led_file = File::create(FILE_PWR_LED).expect(format!("{}Error while opening {}{}", RED, FILE_PWR_LED, C_RESET).as_str());

    match status {
        Status::ON => {
            led_file.write_all("1".as_bytes()).expect(format!("{}Error while writing to {}{}", RED, FILE_PWR_LED, C_RESET).as_str());
            println!("{BOLD}{RED}Power-LED {WHITE}was {GREEN}activated");
        },
        Status::OFF => {
            led_file.write_all("0".as_bytes()).expect(format!("{}Error while writing to {}{}", RED, FILE_PWR_LED, C_RESET).as_str());
            println!("{BOLD}{RED}Power-LED {WHITE}was {RED}deactivated");
        },
    }
}
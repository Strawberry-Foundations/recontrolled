use std::fs::File;
use std::io::Read;

use crate::vars::colors::{BOLD, C_RESET, CYAN, GREEN, LIGHT_GREEN, RED, WHITE};
use crate::vars::constants::{RECONTROLLED_STR, FILE_ACT_LED, FILE_PWR_LED};
use crate::vars::modules::LED;

pub fn print_status() {
    println!(
        "         {RECONTROLLED_STR}

          LED-Status
* -------------------------- *
|                            |
| {RED}Power-LED {CYAN}   (led1){C_RESET}    {} |
| {LIGHT_GREEN}Activity-LED {CYAN}(led0){C_RESET}    {} |
|                            |
* -------------------------- *
        ",
        check_status(LED::PWR), check_status(LED::ACT)
    )
}

fn check_status(led: LED) -> String {
    let mut pwr_file = match led {
        LED::PWR => File::open(FILE_PWR_LED).expect(format!("{}Error while opening {}{}", RED, FILE_PWR_LED, C_RESET).as_str()),
        LED::ACT => File::open(FILE_ACT_LED).expect(format!("{}Error while opening {}{}", RED, FILE_PWR_LED, C_RESET).as_str())
    };
    let mut pwr_status = String::new();

    let fmt_off = format!("{}{}off{}", BOLD, RED, WHITE);
    let fmt_on = format!("{}{}on {}", BOLD, GREEN, WHITE);

    pwr_file.read_to_string(&mut pwr_status).expect(format!("{}Error while reading {}{}", RED, FILE_PWR_LED, C_RESET).as_str());

    if pwr_status.as_str().trim() == "0" {
        fmt_off
    }
    else if pwr_status.as_str().trim() == "255"  || pwr_status.as_str().trim() == "1" {
        fmt_on
    }
    else {
        fmt_off
    }
}
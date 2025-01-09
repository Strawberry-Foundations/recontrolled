use crate::constants::colors::{BOLD, CYAN, C_RESET, GREEN, LIGHT_GREEN, RED, WHITE};
use crate::constants::RECONTROLLED_STR;
use crate::core::led::{Led, Status};
use crate::core::model::RaspberryPi;

pub fn status(model: Box<dyn RaspberryPi>) {
    let led_map = model.get_led_map();

    let pwr_supported = led_map
        .iter()
        .any(|(led, supported)| *led == Led::Pwr && *supported);

    let act_supported = led_map
        .iter()
        .any(|(led, supported)| *led == Led::Act && *supported);

    let pwr_status = if pwr_supported {
        let status = model.get_led_status(Led::Pwr);

        let fmt_on = format!("{}{}on {}", BOLD, GREEN, WHITE);
        let fmt_off = format!("{}{}off{}", BOLD, RED, WHITE);

        let status_str = match status {
            Status::On => fmt_on,
            Status::Off => fmt_off,
        };
        format!("| {RED}Power-LED {CYAN}   (led1){C_RESET}    {} |\n", status_str)
    } else {
        String::new()
    };

    let act_status = if act_supported {
        let status = model.get_led_status(Led::Act);

        let fmt_on = format!("{}{}on {}", BOLD, GREEN, WHITE);
        let fmt_off = format!("{}{}off{}", BOLD, RED, WHITE);

        let status_str = match status {
            Status::On => fmt_on,
            Status::Off => fmt_off,
        };
        format!("| {LIGHT_GREEN}Activity-LED {CYAN}(led0){C_RESET}    {} |\n", status_str)
    } else {
        String::new()
    };

    println!(
        "         {RECONTROLLED_STR}

          LED-Status
* -------------------------- *
|                            |
{}{}|                            |
* -------------------------- *{C_RESET}
        ",
        pwr_status, act_status
    );
}

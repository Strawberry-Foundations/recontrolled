use crate::constants::colors::{BOLD, C_RESET, GREEN, LIGHT_GREEN, RED, WHITE};
use crate::core::led::{Led, Status};
use crate::core::model::RaspberryPi;

use std::thread;
use std::time::Duration;

pub fn blink_switch(delay: u64, model: Box<dyn RaspberryPi>) {
    let blink_mode = match delay {
        2000 => "Slow ",
        1000 => "",
        500 => "Fast ",
        250 => "Super Fast ",
        _ => "Custom ",
    };

    println!("{BOLD}{WHITE}{blink_mode}Switching Blink mode for {RED}Power-{WHITE} and {LIGHT_GREEN}Activity-LED{WHITE} was {GREEN}activated{C_RESET}");

    loop {
        thread::sleep(Duration::from_millis(delay));
        model.set_led_status(Led::Pwr, Status::On);
        model.set_led_status(Led::Act, Status::Off);

        thread::sleep(Duration::from_millis(delay));
        model.set_led_status(Led::Pwr, Status::Off);
        model.set_led_status(Led::Act, Status::On);
    }
}

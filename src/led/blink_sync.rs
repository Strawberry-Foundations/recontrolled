use crate::constants::colors::{BOLD, C_RESET, GREEN, LIGHT_GREEN, RED, WHITE};
use crate::core::model::RaspberryPi;
use crate::core::led::{Led, Status};
use crate::panic_err;

use std::thread;
use std::time::Duration;

pub fn blink_sync(delay: u64, model: Box<dyn RaspberryPi>) {
    if !model.supports_led(Led::Act) || !model.supports_led(Led::Pwr) {
        panic_err!("{BOLD}{RED}Error: {WHITE}The selected LED is not supported by the current model{C_RESET}");
    }
    
    let blink_mode = match delay {
        2000 => "Slow ",
        1000 => "",
        500 => "Fast ",
        250 => "Super Fast ",
        _ => "Custom ",
    };

    println!("{BOLD}{WHITE}{blink_mode}Synchronized Blink mode for {RED}Power-{WHITE} and {LIGHT_GREEN}Activity-LED{WHITE} was {GREEN}activated{C_RESET}");

    loop {
        thread::sleep(Duration::from_millis(delay));
        model.set_led_status(Led::Pwr, Status::On);
        model.set_led_status(Led::Act, Status::On);

        thread::sleep(Duration::from_millis(delay));
        model.set_led_status(Led::Pwr, Status::Off);
        model.set_led_status(Led::Act, Status::Off);
    }
}

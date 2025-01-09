use crate::constants::colors::{BOLD, C_RESET, GREEN, LIGHT_GREEN, RED, WHITE};
use crate::core::led::{Led, Status};
use crate::core::model::RaspberryPi;

use std::thread;
use std::time::Duration;
use crate::panic_err;

pub fn blink(led: Led, delay: u64, model: Box<dyn RaspberryPi>) {
    if !model.supports_led(led) {
        panic_err!("{BOLD}{RED}Error: {WHITE}The selected LED is not supported by the current model{C_RESET}");
    }
    
    let blink_mode = match delay {
        2000 => "Slow ",
        1000 => "",
        500 => "Fast ",
        250 => "Super Fast ",
        _ => "Custom ",
    };

    let led_string = match led {
        Led::Pwr => format!("{}Power-LED", RED),
        Led::Act => format!("{}Activity-LED", LIGHT_GREEN),
    };

    println!("{BOLD}{WHITE}{blink_mode}Blink mode for {led_string}{WHITE} was {GREEN}activated{C_RESET}");

    loop {
        thread::sleep(Duration::from_millis(delay));
        model.set_led_status(led, Status::On);

        thread::sleep(Duration::from_millis(delay));
        model.set_led_status(led, Status::Off);
    }
}

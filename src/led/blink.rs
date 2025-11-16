use crate::constants::colors::{BOLD, C_RESET, GREEN, LIGHT_GREEN, RED, WHITE};
use crate::core::led::{Led, Status};
use crate::led::LEDController;

use crate::panic_err;
use std::thread;
use std::time::Duration;

const fn get_blink_mode(delay: u64) -> &'static str {
    match delay {
        2000 => "Slow ",
        1000 => "",
        500 => "Fast ",
        250 => "Super Fast ",
        _ => "Custom ",
    }
}

fn validate_led_support(controller: &LEDController, led: Led) {
    if !controller.hardware.supports_led(led) {
        panic_err!(
            "{BOLD}{RED}Error: {WHITE}The selected LED is not supported by the current model{C_RESET}"
        );
    }
}

fn validate_both_leds(controller: &LEDController) {
    if !controller.hardware.supports_led(Led::Act) || !controller.hardware.supports_led(Led::Pwr) {
        panic_err!(
            "{BOLD}{RED}Error: {WHITE}The selected LED is not supported by the current model{C_RESET}"
        );
    }
}

impl LEDController {
    pub fn blink(&self, led: Led, delay: u64) {
        validate_led_support(self, led);

        let blink_mode = get_blink_mode(delay);
        let (led_color, led_name) = match led {
            Led::Pwr => (RED, "Power-LED"),
            Led::Act => (LIGHT_GREEN, "Activity-LED"),
        };

        println!(
            "{BOLD}{WHITE}{blink_mode}Blink mode for {led_color}{led_name}{WHITE} was {GREEN}activated{C_RESET}"
        );

        loop {
            thread::sleep(Duration::from_millis(delay));
            self.hardware.set_led_status(led, Status::On);

            thread::sleep(Duration::from_millis(delay));
            self.hardware.set_led_status(led, Status::Off);
        }
    }

    pub fn blink_sync(&self, delay: u64) {
        validate_both_leds(self);

        let blink_mode = get_blink_mode(delay);
        println!(
            "{BOLD}{WHITE}{blink_mode}Synchronized Blink mode for {RED}Power-{WHITE} and {LIGHT_GREEN}Activity-LED{WHITE} was {GREEN}activated{C_RESET}"
        );

        loop {
            thread::sleep(Duration::from_millis(delay));
            self.hardware.set_led_status(Led::Pwr, Status::On);
            self.hardware.set_led_status(Led::Act, Status::On);

            thread::sleep(Duration::from_millis(delay));
            self.hardware.set_led_status(Led::Pwr, Status::Off);
            self.hardware.set_led_status(Led::Act, Status::Off);
        }
    }

    pub fn blink_switch(&self, delay: u64) {
        validate_both_leds(self);

        let blink_mode = get_blink_mode(delay);
        println!(
            "{BOLD}{WHITE}{blink_mode}Switching Blink mode for {RED}Power-{WHITE} and {LIGHT_GREEN}Activity-LED{WHITE} was {GREEN}activated{C_RESET}"
        );

        loop {
            thread::sleep(Duration::from_millis(delay));
            self.hardware.set_led_status(Led::Pwr, Status::On);
            self.hardware.set_led_status(Led::Act, Status::Off);

            thread::sleep(Duration::from_millis(delay));
            self.hardware.set_led_status(Led::Pwr, Status::Off);
            self.hardware.set_led_status(Led::Act, Status::On);
        }
    }
}

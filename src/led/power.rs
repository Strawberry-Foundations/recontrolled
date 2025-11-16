use crate::constants::colors::{BOLD, C_RESET, GREEN, LIGHT_GREEN, RED, WHITE};
use crate::core::led::{Led, Status};
use crate::led::LEDController;
use crate::panic_err;

impl LEDController {
    pub fn set_status(&self, led: Led, status: Status) {
        if !self.hardware.supports_led(led) {
            panic_err!(
                "{BOLD}{RED}Error: {WHITE}The selected LED is not supported by the current model{C_RESET}"
            );
        }

        let led_string = match led {
            Led::Pwr => format!("{RED}Power-LED"),
            Led::Act => format!("{LIGHT_GREEN}Activity-LED"),
        };

        self.hardware.set_led_status(led, status);

        match status {
            Status::On => {
                println!("{BOLD}{led_string} {WHITE}was {GREEN}activated{C_RESET}");
            }
            Status::Off => {
                println!("{BOLD}{led_string} {WHITE}was {RED}deactivated{C_RESET}");
            }
        }
    }
}

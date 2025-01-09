use crate::constants::colors::{BOLD, C_RESET, GREEN, LIGHT_GREEN, RED, WHITE};
use crate::core::led::{Led, Status};
use crate::core::model::RaspberryPi;

pub fn set_status(led: Led, status: Status, model: Box<dyn RaspberryPi>) {
    let led_string = match led {
        Led::Pwr => format!("{}Power-LED", RED),
        Led::Act => format!("{}Activity-LED", LIGHT_GREEN),
    };

    model.set_led_status(led, status);

    match status {
        Status::On => {
            println!("{BOLD}{led_string} {WHITE}was {GREEN}activated{C_RESET}");
        }
        Status::Off => {
            println!("{BOLD}{led_string} {WHITE}was {RED}deactivated{C_RESET}");
        }
    }
}

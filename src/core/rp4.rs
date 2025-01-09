use crate::constants::{FILE_ACT_LED, FILE_PWR_LED};
use crate::core::led::Led;
use crate::core::model::{RaspberryPi, RaspberryPiModel};

#[derive(Debug)]
pub struct RaspberryPi4B {
    pub model: RaspberryPiModel,
    pub led_map: [(Led, bool); 2],
    pub file_pwr_led: &'static str,
    pub file_act_led: &'static str,
}

impl RaspberryPi for RaspberryPi4B {
    fn get_model_name(&self) -> &str {
        self.model.to_raw()
    }

    fn get_led_map(&self) -> &[(Led, bool)] {
        &self.led_map
    }

    fn get_led_file(&self, led: Led) -> &str {
        match led {
            Led::Pwr => self.file_pwr_led,
            Led::Act => self.file_act_led,
        }
    }
}

impl RaspberryPi4B {
    pub fn new() -> Self {
        Self {
            model: RaspberryPiModel::Pi4B,
            led_map: [(Led::Pwr, true), (Led::Act, true)],
            file_pwr_led: FILE_PWR_LED,
            file_act_led: FILE_ACT_LED,
        }
    }
}

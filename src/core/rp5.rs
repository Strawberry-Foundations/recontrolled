use crate::constants::{FILE_ACT_LED, FILE_PWR_LED};
use crate::core::led::Led;
use crate::core::hardware::{RaspberryPi, RaspberryPiModel};

#[derive(Debug)]
pub struct RaspberryPi5B {
    pub model: RaspberryPiModel,
    pub led_map: [(Led, bool); 2],
    pub file_pwr_led: &'static str,
    pub file_act_led: &'static str,
}

impl RaspberryPi for RaspberryPi5B {
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

impl RaspberryPi5B {
    pub const fn new() -> Self {
        Self {
            model: RaspberryPiModel::Pi5B,
            led_map: [(Led::Pwr, true), (Led::Act, true)],
            file_pwr_led: FILE_PWR_LED,
            file_act_led: FILE_ACT_LED,
        }
    }
}

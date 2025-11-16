use crate::constants::FILE_ACT_LED;
use crate::core::led::Led;
use crate::core::hardware::{RaspberryPi, RaspberryPiModel};

#[derive(Debug)]
pub struct RaspberryPiZero2W {
    pub model: RaspberryPiModel,
    pub led_map: [(Led, bool); 2],
    pub file_pwr_led: &'static str,
    pub file_act_led: &'static str,
}

impl RaspberryPi for RaspberryPiZero2W {
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

impl RaspberryPiZero2W {
    pub const fn new() -> Self {
        Self {
            model: RaspberryPiModel::PiZero2W,
            led_map: [(Led::Pwr, false), (Led::Act, true)],
            file_pwr_led: "",
            file_act_led: FILE_ACT_LED,
        }
    }
}

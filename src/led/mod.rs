use crate::core::hardware::RaspberryPi;

pub mod power;
pub mod blink;

pub struct LEDController {
    pub hardware: Box<dyn RaspberryPi>,
}

impl LEDController {
    pub fn new(model: Box<dyn RaspberryPi>) -> Self {
        Self { hardware: model }
    }
}

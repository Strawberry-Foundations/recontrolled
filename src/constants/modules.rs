pub enum Led {
    Act = 0,
    Pwr = 1
}

pub enum Status {
    On = 1,
    Off = 0,
}

#[derive(Debug)]
pub enum RaspberryPiModel {
    Pi4B,
    PiZero2W,
}

impl RaspberryPiModel {
    pub fn model_name(&self) -> &'static str {
        match self {
            RaspberryPiModel::Pi4B => "Raspberry Pi 4 Model B",
            RaspberryPiModel::PiZero2W => "Raspberry Pi Zero 2 W",
        }
    }

    pub fn from_model_name(name: &str) -> Option<Self> {
        match name {
            name if name.starts_with("Raspberry Pi 4 Model B") => Some(RaspberryPiModel::Pi4B),
            name if name.starts_with("Raspberry Pi Zero 2 W") => Some(RaspberryPiModel::PiZero2W),
            _ => None,
        }
    }
}
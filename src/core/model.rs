#![allow(dead_code)]

use crate::constants::colors::{C_RESET, RED};
use crate::core::led::{Led, Status};
use crate::panic_err;

use std::fs::File;
use std::io::{Read, Write};

pub fn get_raspberry_pi_model() -> Option<String> {
    use std::fs;

    fs::read_to_string("/proc/cpuinfo").ok().and_then(|cpuinfo| {
        cpuinfo
            .lines()
            .find(|line| line.starts_with("Model"))
            .map(|line| line.replace("Model\t\t: ", ""))
    })
}

#[derive(Debug)]
pub enum RaspberryPiModel {
    PiZero2W,
    Pi4B,
    Pi5B,
}

impl RaspberryPiModel {
    pub fn to_raw(&self) -> &str {
        match self {
            RaspberryPiModel::PiZero2W => "Raspberry Pi Zero 2 W",
            RaspberryPiModel::Pi4B => "Raspberry Pi 4 Model B",
            RaspberryPiModel::Pi5B => "Raspberry Pi 5 Model B",
        }
    }

    pub fn from_raw(model: &str) -> Option<Self> {
        match model {
            name if name.starts_with("Raspberry Pi Zero 2 W") => Some(RaspberryPiModel::PiZero2W),
            name if name.starts_with("Raspberry Pi 4 Model B") => Some(RaspberryPiModel::Pi4B),
            name if name.starts_with("Raspberry Pi 5 Model B") => Some(RaspberryPiModel::Pi5B),
            _ => None,
        }
    }
}

pub trait RaspberryPi {
    fn get_model_name(&self) -> &str;
    fn get_led_map(&self) -> &[(Led, bool)];
    fn get_led_file(&self, led: Led) -> &str;

    fn get_led_status(&self, led: Led) -> Status {
        let file_path = self.get_led_file(led);
        let mut led_file = File::open(file_path)
            .unwrap_or_else(|err| panic_err!("{RED}Error while opening {file_path}: {err}{C_RESET}"));

        let mut led_status = String::new();

        led_file
            .read_to_string(&mut led_status)
            .unwrap_or_else(|err| panic_err!("{RED}Error while reading {file_path}: {err}{C_RESET}"));

        match led_status.as_str().trim() {
            "255" | "1" => Status::On,
            _ => Status::Off,
        }
    }

    fn get_led_object(&self, led: Led) -> File {
        let file = self.get_led_file(led);
        File::open(file)
            .unwrap_or_else(|err| panic_err!("{RED}Error while opening {file}: {err}{C_RESET}"))
    }

    fn write_to_led(&self, mut file: &File, file_path: &str, status: Status) {
        let status = match status {
            Status::On => "1",
            Status::Off => "0",
        };

        file
            .write_all(status.as_bytes())
            .unwrap_or_else(|err| panic_err!("{RED}Error while writing to {file_path}: {err}{C_RESET}"));
    }

    fn set_led_status(&self, led: Led, status: Status) {
        let file_path = self.get_led_file(led);
        let mut led_file = File::create(file_path)
            .unwrap_or_else(|err| panic_err!("{RED}Error while opening {file_path}: {err}{C_RESET}"));

        let status = match status {
            Status::On => "1",
            Status::Off => "0",
        };

        led_file
            .write_all(status.as_bytes())
            .unwrap_or_else(|err| panic_err!("{RED}Error while writing to {file_path}: {err}{C_RESET}"));
    }
    
    fn supports_led(&self, led: Led) -> bool {
        self.get_led_map()
            .iter()
            .any(|(l, supported)| *l == led && *supported)
    }
}

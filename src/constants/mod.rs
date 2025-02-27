pub mod colors;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_DATE: &str = "2025-01-09";

pub const FILE_PWR_LED: &str = "/sys/class/leds/PWR/brightness";
pub const FILE_ACT_LED: &str = "/sys/class/leds/ACT/brightness";

pub const RECONTROLLED_STR: &str = "\x1b[1m\x1b[96mR\x1b[32me\x1b[33mc\x1b[34mo\x1b[35mn\x1b[36mt\x1b[94mr\x1b[31mo\x1b[92ml\x1b[95ml\x1b[92me\x1b[33md\x1b[0m\x1b[1m\x1b[37m";
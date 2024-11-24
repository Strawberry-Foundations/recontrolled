use std::fs;

pub fn get_raspberry_pi_model() -> Option<String> {
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("Model") {
                return Some(line.replace("Model\t\t: ", ""));
            } else if line.starts_with("Hardware") {
                return Some(line.replace("Hardware\t: ", ""));
            }
        }
    }
    None
}


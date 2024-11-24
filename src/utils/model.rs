pub fn get_raspberry_pi_model() -> Option<String> {
    use std::fs;

    fs::read_to_string("/proc/cpuinfo").ok().and_then(|cpuinfo| {
        cpuinfo
            .lines()
            .find(|line| line.starts_with("Model"))
            .map(|line| line.replace("Model\t\t: ", ""))
    })
}
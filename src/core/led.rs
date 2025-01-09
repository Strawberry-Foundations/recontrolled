#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Led {
    Act,
    Pwr
}

#[derive(Debug, Copy, Clone)]
pub enum Status {
    On,
    Off,
}
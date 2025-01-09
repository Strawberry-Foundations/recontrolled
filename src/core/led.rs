#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Led {
    Act,
    Pwr
}

#[derive(Debug)]
pub enum Status {
    On,
    Off,
}
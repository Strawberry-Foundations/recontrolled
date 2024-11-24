#[macro_export]
macro_rules! panic_err {
    ($($arg:tt)*) => {
        let res = std::fmt::format(format_args!($($arg)*));
        eprintln!("{}", res);
        std::process::exit(1);
    }
}

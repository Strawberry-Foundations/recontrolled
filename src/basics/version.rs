use crate::vars::constants::{BUILD_DATE, RECONTROLLED_STR, VERSION};

pub fn print_version() {
    println!(
        "         {RECONTROLLED_STR}
* -------------------------- *
|     v{VERSION} - {BUILD_DATE}    |
|                            |
|  by Strawberry Foundations |
|                            |
|        {}.       |
* -------------------------- *
        ",
        RECONTROLLED_STR.to_lowercase()
    )
}
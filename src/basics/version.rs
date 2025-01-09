use crate::constants::colors::C_RESET;
use crate::constants::{BUILD_DATE, RECONTROLLED_STR, VERSION};

pub fn version() {
    println!(
        "         {RECONTROLLED_STR}
* -------------------------- *
|     v{VERSION} - {BUILD_DATE}    |
|                            |
|  by Strawberry Foundations |
|                            |
|        {}.       |
* -------------------------- *{C_RESET}
        ",
        RECONTROLLED_STR.to_lowercase()
    )
}
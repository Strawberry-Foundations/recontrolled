use crate::constants::colors::{BOLD, YELLOW, C_RESET, WHITE, RED, UNDERLINE, LIGHT_GREEN, GREEN, CYAN};
use crate::constants::RECONTROLLED_STR;
use crate::core::hardware::RaspberryPi;

pub fn about(model: Box<dyn RaspberryPi>) {
    println!(
        "           {RECONTROLLED_STR}
* ------------------------------ *
|    Copyright (C) 2022 - 2025   |
|     Strawberry Foundations     |
|                                |
|       All rights reserved      |
|       Made with Rust Lang      |
* ------------------------------ *

              {BOLD}{YELLOW}WARNING{C_RESET}{WHITE}{BOLD}
* ------------------------------ *
|    This program comes with     |
|     absolutely {RED}{UNDERLINE}NO{C_RESET}{WHITE}{BOLD} warranty     |
|                                |
| {LIGHT_GREEN}This is free software, and you{C_RESET}{WHITE}{BOLD} |
| {LIGHT_GREEN}are welcome to redistribute it{C_RESET}{WHITE}{BOLD} |
* ------------------------------ *

                {BOLD}{GREEN}License{C_RESET}{WHITE}{BOLD}
* ----------------------------------- *
|                 MIT                 |
|         Open Source License         |
|                                     |
| https://opensource.org/license/mit/ |
* ----------------------------------- *{C_RESET}

{BOLD}{C_RESET}Running on {CYAN}{}{C_RESET}{WHITE}
        ",
        model.get_model_name()
    );
}

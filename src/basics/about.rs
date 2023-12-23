use crate::vars::colors::*;
use crate::vars::constants::RECONTROLLED_STR;

pub fn print_about() {
    println!(
        "           {RECONTROLLED_STR}
* ------------------------------ *
|    Copyright (C) 2022 - 2024   |
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
* ----------------------------------- *
        "
    )
}
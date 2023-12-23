use crate::vars::colors::*;
use crate::vars::constants::RECONTROLLED_STR;

pub fn print_about() {
    println!(
        "           {RECONTROLLED_STR}
* ------------------------------ *
| Copyright (C) 2023 Juliandev02 |
|       {RECONTROLLED_STR} 2022        |
|                                |
|       All rights reserved      |
|    Made with the C Language    |
* ------------------------------ *

              {YELLOW}WARNING{C_RESET}{WHITE}{BOLD}
* ------------------------------ *
|    This program comes with     |
|     absolutely {RED}{UNDERLINE}NO warranty     |
|                                |
| This is free software, and you |
| are welcome to redistribute it |
* ------------------------------ *

                License
* ----------------------------------- *
|                 MIT                 |
|         Open Source License         |
|                                     |
| https://opensource.org/license/mit/ |
* ----------------------------------- *
        "
    )
}
use crate::colors::*;

pub fn print_help() {

    println!(
        "               {}{}* -------- {}R{}e{}c{}o{}n{}t{}r{}o{}l{}l{}e{}d{}{}{} -------- *\n
             {BOLD}{PURPLE}Usage:{WHITE} recontrolled {CYAN}[command] {RED}[<options>]{C_RESET}\n

            {BOLD}{BLUE}Basics                      {RED}Power-LED (pwr){C_RESET}
{BOLD}{WHITE}* {DUMMY               }--------------------------- + -------------------------------- *
{BOLD}{WHITE}| {LIGHT_MAGENTA}help: Displays help        {WHITE} | {CYAN}ar: Activates Pwr-LED           {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}version: Displays version  {WHITE} | {CYAN}dr: Deactivated Pwr-LED         {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}about: About Recontrolled  {WHITE} | {CYAN}br: Blinks Pwr-LED              {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}status: Shows status       {WHITE} | {CYAN}brf: Blinks Pwr-LED fast        {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}        of all leds        {WHITE} | {CYAN}brs: Blinks Pwr-LED slow        {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}                           {WHITE} | {CYAN}brff: Blinks Pwr-LED super fast {WHITE} |
{BOLD}{WHITE}* {DUMMY               }--------------------------- + -------------------------------- *

           {BOLD}{YELLOW}All LED's                   {LIGHT_GREEN}Activity-LED (act){C_RESET}
{BOLD}{WHITE}* {DUMMY             }--------------------------- + -------------------------------- *
{BOLD}{WHITE}| {LIGHT_GREEN}lb: Blink LEDs synchronized{WHITE} | {PURPLE}ag: Activates Act-LED           {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}lbs: Blink LEDs in switch  {WHITE} | {PURPLE}dg: Deactivated Act-LED         {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}                           {WHITE} | {PURPLE}bg: Blinks Act-LED              {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}                           {WHITE} | {PURPLE}bgf: Blinks Act-LED fast        {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}                           {WHITE} | {PURPLE}bgs: Blinks Act-LED slow        {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}                           {WHITE} | {PURPLE}bgff: Blinks Act-LED super fast {WHITE} |
{BOLD}{WHITE}* {DUMMY             }--------------------------- + -------------------------------- *
        ",
        BOLD, WHITE,
            LIGHT_CYAN, GREEN, YELLOW, BLUE, PURPLE, CYAN, LIGHT_BLUE, RED, LIGHT_GREEN, LIGHT_MAGENTA, LIGHT_GREEN, YELLOW,
        C_RESET, BOLD, WHITE
    )
}
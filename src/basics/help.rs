use crate::constants::colors::*;
use crate::constants::constants::RECONTROLLED_STR;

pub fn print_help() {

    println!(
        "               {}{}* -------- {RECONTROLLED_STR} -------- *\n
             {BOLD}{PURPLE}Usage:{WHITE} recontrolled {CYAN}[command] {RED}[<options>]{C_RESET}\n

             {BOLD}{BLUE}Basics                       {RED}Power-LED (pwr){C_RESET}
{BOLD}{WHITE}* {DUMMY               }----------------------------- + ------------------------------- *
{BOLD}{WHITE}| {LIGHT_MAGENTA}help: Displays help          {WHITE} | {CYAN}ar: Activates Pwr-LED          {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}version: Displays version    {WHITE} | {CYAN}dr: Deactivated Pwr-LED        {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}about: About Recontrolled    {WHITE} | {CYAN}br: Blinks Pwr-LED             {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}status: Shows status ...     {WHITE} | {CYAN}brf: Blinks Pwr-LED fast       {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA} of all leds                 {WHITE} | {CYAN}brs: Blinks Pwr-LED slow       {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}                             {WHITE} | {CYAN}brff: Blinks Pwr-LED super fast{WHITE} |
{BOLD}{WHITE}* {DUMMY               }----------------------------- + ------------------------------- *

            {BOLD}{YELLOW}All LED's                    {LIGHT_GREEN}Activity-LED (act){C_RESET}
{BOLD}{WHITE}* {DUMMY             }----------------------------- + ------------------------------- *
{BOLD}{WHITE}| {LIGHT_GREEN}lb: Blink LEDs synchronized  {WHITE} | {PURPLE}ag: Activates Act-LED          {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}lbs: Blink LEDs in switch    {WHITE} | {PURPLE}dg: Deactivated Act-LED        {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}lbc <time>: Blink LEDs  ...  {WHITE} | {PURPLE}bg: Blinks Act-LED             {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}  synchronized (custom delay){WHITE} | {PURPLE}bgf: Blinks Act-LED fast       {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}lbsc <time>: Blink LEDs ...  {WHITE} | {PURPLE}bgs: Blinks Act-LED slow       {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}  in switch (custom delay)   {WHITE} | {PURPLE}bgff: Blinks Act-LED super fast{WHITE} |
{BOLD}{WHITE}* {DUMMY             }----------------------------- + -------------------------------{C_RESET}*
        ",
        BOLD, WHITE
    )
}
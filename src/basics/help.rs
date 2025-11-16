use crate::constants::RECONTROLLED_STR;
use crate::constants::colors::{
    BLUE, BOLD, C_RESET, CYAN, DUMMY, LIGHT_GREEN, LIGHT_MAGENTA, PURPLE, RED, WHITE, YELLOW,
};

pub fn print_help() {
    println!(
        "                    {BOLD}{WHITE}* -------- {RECONTROLLED_STR} -------- *
                {BOLD}{PURPLE}Usage:{WHITE} recontrolled {CYAN}[command] {RED}[<options>]{C_RESET}

               {BOLD}{BLUE}Basics                           {RED}Power-LED (pwr){C_RESET}
{BOLD}{WHITE}* {DUMMY               }--------------------------------- + ----------------------------------- *
{BOLD}{WHITE}| {LIGHT_MAGENTA}help: Displays help             {WHITE}  | {CYAN}pwr on: Activates Pwr-LED          {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}version/ver: Displays version   {WHITE}  | {CYAN}pwr off: Deactivates Pwr-LED       {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}about: About Recontrolled       {WHITE}  | {CYAN}pwr blink: Blinks Pwr-LED          {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}status: Shows status of all LEDs{WHITE}  | {CYAN}pwr blink_fast: Fast blink         {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}                                {WHITE}  | {CYAN}pwr blink_slow: Slow blink         {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}                                {WHITE}  | {CYAN}pwr blink_super_fast: Super fast   {WHITE} |
{BOLD}{WHITE}| {LIGHT_MAGENTA}                                {WHITE}  | {CYAN}pwr blink_custom <ms>: Custom delay{WHITE} |
{BOLD}{WHITE}* {DUMMY               }--------------------------------- + ----------------------------------- *

              {BOLD}{YELLOW}All LEDs                         {LIGHT_GREEN}Activity-LED (act){C_RESET}
{BOLD}{WHITE}* {DUMMY             }--------------------------------- + ----------------------------------- *
{BOLD}{WHITE}| {LIGHT_GREEN}blink_sync: Blink LEDs sync      {WHITE} | {PURPLE}act on: Activates Act-LED          {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}blink_switch: Blink LEDs switch  {WHITE} | {PURPLE}act off: Deactivates Act-LED       {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}blink_sync_custom <ms>: Sync ... {WHITE} | {PURPLE}act blink: Blinks Act-LED          {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}  with custom delay              {WHITE} | {PURPLE}act blink_fast: Fast blink         {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}blink_switch_custom <ms>: ...    {WHITE} | {PURPLE}act blink_slow: Slow blink         {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}  Switch with custom delay       {WHITE} | {PURPLE}act blink_super_fast: Super fast   {WHITE} |
{BOLD}{WHITE}| {LIGHT_GREEN}                                 {WHITE} | {PURPLE}act blink_custom <ms>: Custom delay{WHITE} |
{BOLD}{WHITE}* {DUMMY             }--------------------------------- + ----------------------------------- {C_RESET}*
        "
    );
}

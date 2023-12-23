#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "help.h"
#include "vars/colors.h"

int help() {
    printf(BOLD WHITE "               * -------- " BOLD LIGHT_CYAN"R" GREEN "e" YELLOW "c" BLUE "o" MAGENTA "n" CYAN "t" LIGHT_BLUE "r" RED "o" LIGHT_GREEN "l" LIGHT_MAGENTA "l" LIGHT_GREEN "e" YELLOW "d" RESET BOLD WHITE " -------- *\n\n");
 // printf(BOLD WHITE "                             * " BOLD LIGHT_CYAN "B" GREEN "E" YELLOW "" BLUE "T" MAGENTA"A" WHITE BOLD" *                         \n\n");
    printf(BOLD MAGENTA "             Usage: " WHITE "recontrolled" CYAN " [command] " RED "[<option>]      \n\n" RESET);
    printf(BOLD WHITE "            " BLUE "Basics                      " RED "Power-LED (pwr)      \n");
    printf(BOLD WHITE "* -------------------------- + -------------------------------- *\n");
    printf(BOLD WHITE "| " LIGHT_MAGENTA "help: Displays help" WHITE "        | " CYAN "ar: Activates Pwr-LED" WHITE "            |\n");
    printf(BOLD WHITE "| " LIGHT_MAGENTA "ver: Displays version" WHITE "      | " CYAN "dr: Deactivated Pwr-LED" WHITE "          |\n");
    printf(BOLD WHITE "| " LIGHT_MAGENTA "about: About Recontrolled" WHITE "  | " CYAN "br: Blinks Pwr-LED" WHITE "               |\n");
    printf(BOLD WHITE "| " LIGHT_MAGENTA "status: Shows status       " WHITE "| " CYAN "brf: Blinks Pwr-LED fast" WHITE "         |\n");
    printf(BOLD WHITE "| " LIGHT_MAGENTA "        of all leds      " WHITE "  | " CYAN "brs: Blinks Pwr-LED slow" WHITE "         |\n");
    printf(BOLD WHITE "| " LIGHT_MAGENTA "                         " WHITE "  | " CYAN "brff: Blinks Pwr-LED super fast" WHITE "  |\n");
    printf(BOLD WHITE "* -------------------------- + -------------------------------- *\n");
    printf(BOLD WHITE "                                                         \n");
    printf(BOLD WHITE "           " YELLOW "All LED's                   " LIGHT_GREEN "Activity-LED (act)    \n");
    printf(BOLD WHITE "* -------------------------- + -------------------------------- *\n");
    printf(BOLD WHITE "| " LIGHT_GREEN "lb: Blink LEDs syncronized" WHITE " | " MAGENTA "ag: Activates Act-LED" WHITE "            |\n");
    printf(BOLD WHITE "| " LIGHT_GREEN "lbs: Blink LEDs in switch" WHITE "  | " MAGENTA "dg: Deactivated Act-LED" WHITE "          |\n");
    printf(BOLD WHITE "| " LIGHT_GREEN "                         " WHITE "  | " MAGENTA "bg: Blinks Act-LED" WHITE "               |\n");
    printf(BOLD WHITE "| " LIGHT_GREEN "                         " WHITE "  | " MAGENTA "bgf: Blinks Act-LED fast" WHITE "         |\n");
    printf(BOLD WHITE "| " LIGHT_GREEN "                         " WHITE "  | " MAGENTA "bgs: Blinks Act-LED slow" WHITE "         |\n");
    printf(BOLD WHITE "| " LIGHT_GREEN "                         " WHITE "  | " MAGENTA "bgff: Blinks Act-LED super fast" WHITE "  |\n");
    printf(BOLD WHITE "* -------------------------- + -------------------------------- *\n");

    return 0;
}
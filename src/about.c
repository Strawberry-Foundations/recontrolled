#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "about.h"
#include "vars/colors.h"
#include "vars/build_config.h"

int about() {
    printf(BOLD WHITE "           " BOLD LIGHT_CYAN"R" GREEN "e" YELLOW "c" BLUE "o" MAGENTA "n" CYAN "t" LIGHT_BLUE "r" RED "o" LIGHT_GREEN "l" LIGHT_MAGENTA "l" LIGHT_GREEN "e" YELLOW "d" RESET BOLD WHITE "\n");
    printf(BOLD WHITE "* ------------------------------ *\n");
    printf(BOLD WHITE "| Copyright (C) 2023 Juliandev02 |\n");
    printf(BOLD WHITE "|       " BOLD LIGHT_CYAN"R" GREEN "e" YELLOW "c" BLUE "o" MAGENTA "n" CYAN "t" LIGHT_BLUE "r" RED "o" LIGHT_GREEN "l" LIGHT_MAGENTA "l" LIGHT_GREEN "e" YELLOW "d" WHITE " 2022        |\n");
    printf(BOLD WHITE "|                                |\n");
    printf(BOLD WHITE "|       All rights reserved      |\n");
    printf(BOLD WHITE "|    Made with the C Language    |\n");
    printf(BOLD WHITE "* ------------------------------ *\n\n");

    printf(BOLD WHITE "              " BOLD YELLOW "WARNING             \n");
    printf(BOLD WHITE "* ------------------------------ *\n");
    printf(BOLD WHITE "|    This program comes with     |\n");
    printf(BOLD WHITE "|     absolutely " UNDERLINE RED "NO" RESET BOLD WHITE" warranty     |\n");
    printf(BOLD WHITE "|                                |\n");
    printf(BOLD WHITE "| " LIGHT_GREEN "This is free software, and you" RESET BOLD WHITE" |\n");
    printf(BOLD WHITE "| " LIGHT_GREEN "are welcome to redistribute it" RESET BOLD WHITE" |\n");
    printf(BOLD WHITE "* ------------------------------ *\n\n");

    printf(BOLD WHITE "                " BOLD GREEN "License             \n");
    printf(BOLD WHITE "* ----------------------------------- *\n");
    printf(BOLD WHITE "|                 MIT                 |\n");
    printf(BOLD WHITE "|         Open Source License         |\n");
    printf(BOLD WHITE "|                                     |\n");
    printf(BOLD WHITE "| https://opensource.org/license/mit/ |\n");
    printf(BOLD WHITE "* ----------------------------------- *\n\n");

    return 0;
}
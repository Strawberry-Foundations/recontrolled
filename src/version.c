#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "version.h"
#include "vars/colors.h"
#include "vars/build_config.h"

int version() {
    printf(BOLD WHITE "         " BOLD LIGHT_CYAN"R" GREEN "e" YELLOW "c" BLUE "o" MAGENTA "n" CYAN "t" LIGHT_BLUE "r" RED "o" LIGHT_GREEN "l" LIGHT_MAGENTA "l" LIGHT_GREEN "e" YELLOW "d" RESET BOLD WHITE "\n");
 // printf(BOLD WHITE "           * " BOLD LIGHT_CYAN "B" GREEN "E" YELLOW "" BLUE "T" MAGENTA"A" WHITE BOLD" *                         \n");
    printf(BOLD WHITE "* -------------------------- *\n");
    printf(BOLD WHITE "|      v" VERSION " - "BUILD_DATE"     |\n");
    printf(BOLD WHITE "|                            |\n");
    printf(BOLD WHITE "|   -> GitHub: Juliandev02   |\n");
    printf(BOLD WHITE "|                            |\n");
    printf(BOLD WHITE "|        " BOLD LIGHT_CYAN"r" GREEN "e" YELLOW "c" BLUE "o" MAGENTA "n" CYAN "t" LIGHT_BLUE "r" RED "o" LIGHT_GREEN "l" LIGHT_MAGENTA "l" LIGHT_GREEN "e" YELLOW "d" WHITE ".       |\n");
    printf(BOLD WHITE "* -------------------------- *\n");
    return 0;
}
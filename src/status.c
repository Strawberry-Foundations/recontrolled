#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "status.h"
#include "recontrolled.h"
#include "vars/colors.h"
#include "vars/sysfiles.h"

int status() {
    printf(BOLD WHITE "         " BOLD LIGHT_CYAN"R" GREEN "e" YELLOW "c" BLUE "o" MAGENTA "n" CYAN "t" LIGHT_BLUE "r" RED "o" LIGHT_GREEN "l" LIGHT_MAGENTA "l" LIGHT_GREEN "e" YELLOW "d" RESET BOLD WHITE "\n\n");

    printf(BOLD WHITE "          LED-Status          \n");
    printf(BOLD WHITE "* -------------------------- *\n");
    printf(BOLD WHITE "|                            |\n");
    check_red();
    check_green();
    printf(BOLD WHITE "|                            |\n");
    printf(BOLD WHITE "* -------------------------- *\n");
    return 0;
}

int check_red() {
    int valofled1;
    FILE *led1;
    led1 = fopen(RED_LED, "r");

    if(led1 == NULL) {
        printf(RED BOLD "Huh? An Error ocurred!\n");   
        exit(1);             
    }

    fscanf(led1,"%d", &valofled1);
    fclose(led1);

    if(valofled1 == 0) {
        printf(BOLD WHITE "| " RED "Power-LED " CYAN "(led1)       " RED "off " WHITE"|\n");
    }
    else if (valofled1 == 255 || valofled1 == 1) {
        printf(BOLD WHITE "| " RED "Power-LED " CYAN "(led1)       " GREEN "on  " WHITE"|\n");
    }
    
    return 0;
}

int check_green() {
    int valofled0;
    FILE *led0;
    led0 = fopen(GREEN_LED, "r");

    if(led0 == NULL) {
        printf(RED BOLD "Huh? An Error ocurred!\n");   
        exit(1);             
    }

    fscanf(led0,"%d", &valofled0);
    fclose(led0);

    if(valofled0 == 0) {
        printf(BOLD WHITE "| " LIGHT_GREEN "Activity-LED " CYAN "(led0)    " RED "off " WHITE"|\n");
    }
    else if (valofled0 == 255 || valofled0 == 1) {
        printf(BOLD WHITE "| " LIGHT_GREEN "Activity-LED " CYAN "(led0)    " GREEN "on  " WHITE"|\n");
    }
    
    return 0;
}
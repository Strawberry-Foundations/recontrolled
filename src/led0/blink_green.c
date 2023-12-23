#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <unistd.h>

#include "blink_green.h"
#include "../vars/colors.h"
#include "../vars/sysfiles.h"

int blink_green() {
    printf(BOLD WHITE "Blink mode for " LIGHT_GREEN "Activity LED " BOLD WHITE "was " BOLD GREEN "activated\n");

    FILE *file;
    file = fopen(GREEN_LED, "w");

    if(file == NULL) {
        printf(RED BOLD "Please run recontrolled as root :(\n");   
        exit(1);             
    }

    while(1) {
    file = fopen(GREEN_LED, "w");

    fprintf(file, "%d", 1);
    fclose(file);

    sleep(1);

    file = fopen(GREEN_LED, "w");

    fprintf(file, "%d", 0);
    fclose(file);

    sleep(1);
    }

    return 0;
}
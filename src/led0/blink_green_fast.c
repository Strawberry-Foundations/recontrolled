#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <unistd.h>

#include "blink_green_fast.h"
#include "../vars/colors.h"
#include "../vars/sysfiles.h"
#include "../msleep.h"

int blink_green_fast() {
    printf(BOLD WHITE "Fast Blink mode for " LIGHT_GREEN "Activity LED " BOLD WHITE "was " BOLD GREEN "activated\n");

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

    msleep(500);

    file = fopen(GREEN_LED, "w");

    fprintf(file, "%d", 0);
    fclose(file);

    msleep(500);
    }

    return 0;
}
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <unistd.h>

#include "blink_red_fast.h"
#include "../vars/colors.h"
#include "../vars/sysfiles.h"
#include "../msleep.h"

int blink_red_fast() {
    printf(BOLD WHITE "Fast Blink mode for " RED "Power LED " BOLD WHITE "was " BOLD GREEN "activated\n");

    FILE *file;
    file = fopen(RED_LED, "w");

    if(file == NULL) {
        printf(RED BOLD "Please run recontrolled as root :(\n");   
        exit(1);             
    }

    while(1) {
    file = fopen(RED_LED, "w");

    fprintf(file, "%d", 1);
    fclose(file);

    msleep(500);

    file = fopen(RED_LED, "w");

    fprintf(file, "%d", 0);
    fclose(file);

    msleep(500);
    }

    return 0;
}
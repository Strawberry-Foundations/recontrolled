#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <unistd.h>

#include "blink_red_slow.h"
#include "../vars/colors.h"
#include "../vars/sysfiles.h"
#include "../msleep.h"

int blink_red_slow() {
    printf(BOLD WHITE "Slow Blink mode for " RED "Power LED " BOLD WHITE "was " BOLD GREEN "activated\n");

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

    sleep(2);

    file = fopen(RED_LED, "w");

    fprintf(file, "%d", 0);
    fclose(file);

    sleep(2);
    }

    return 0;
}
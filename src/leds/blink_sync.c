#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <unistd.h>

#include "blink_sync.h"
#include "../vars/colors.h"
#include "../vars/sysfiles.h"

int blink_sync() {
    printf(BOLD WHITE "Syncronized Blink mode for " RED "Power " WHITE "and " LIGHT_GREEN "Activity LED " BOLD WHITE "was " BOLD GREEN "activated\n");

    FILE *led0; 
    led0 = fopen(GREEN_LED, "w");

    FILE *led1;
    led1 = fopen(RED_LED, "w");

    if(led0 == NULL) {
        printf(RED BOLD "Please run recontrolled as root :(\n");   
        exit(1);             
    }

    while(1) {
    led0 = fopen(GREEN_LED, "w");
    led1 = fopen(RED_LED, "w");

    fprintf(led0, "%d", 1);
    fprintf(led1, "%d", 1);

    fclose(led0);
    fclose(led1);

    sleep(1);

    led0 = fopen(GREEN_LED, "w");
    led1 = fopen(RED_LED, "w");

    fprintf(led0, "%d", 0);
    fprintf(led1, "%d", 0);

    fclose(led0);
    fclose(led1);

    sleep(1);
    }

    return 0;
}
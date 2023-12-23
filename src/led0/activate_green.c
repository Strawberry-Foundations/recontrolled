#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "activate_green.h"
#include "../vars/colors.h"
#include "../vars/sysfiles.h"

int activate_green() {
    FILE *file; 
    file = fopen(GREEN_LED, "w");

    if(file == NULL) {
        printf(RED BOLD "Please run recontrolled as root :(\n");   
        exit(1);             
    }

    fprintf(file, "%d", 1);
    fclose(file);

    printf(BOLD LIGHT_GREEN "Activity-LED " BOLD WHITE "was " BOLD GREEN "activated\n");

    return 0;
}
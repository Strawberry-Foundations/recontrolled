#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "deactivate_green.h"
#include "../vars/colors.h"
#include "../vars/sysfiles.h"

int deactivate_green() {
    FILE *file; 
    file = fopen(GREEN_LED, "w");

    if(file == NULL) {
        printf(RED BOLD "Please run recontrolled as root :(\n");   
        exit(1);             
    }

    fprintf(file, "%d", 0);
    fclose(file);

    printf(BOLD LIGHT_GREEN "Activity-LED " BOLD WHITE "was " BOLD RED "deactivated\n");

    return 0;
}
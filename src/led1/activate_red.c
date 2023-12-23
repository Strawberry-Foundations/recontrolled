#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "activate_red.h"
#include "../vars/colors.h"
#include "../vars/sysfiles.h"

int activate_red() {
    FILE *file; 
    file = fopen(RED_LED, "w");

    if(file == NULL) {
        printf(RED BOLD "Please run recontrolled as root :(\n");   
        exit(1);             
    }

    fprintf(file, "%d", 1);
    fclose(file);

    printf(BOLD RED "Power-LED " BOLD WHITE "was " BOLD GREEN "activated\n");

    return 0;
}
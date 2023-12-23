#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "recontrolled.h"

int main(int argc, char* argv[]) {
    if (argc == 1)
        help();
    
    else if ((!strcmp(argv[1], "help") && argc == 2))
        return help();

    else if ((!strcmp(argv[1], "version") && argc == 2) || (!strcmp(argv[1], "ver")  && argc == 2))
        return version();
    
    else if ((!strcmp(argv[1], "about") && argc == 2))
        return about();

    else if ((!strcmp(argv[1], "status") && argc == 2))
        return status();


    else if ((!strcmp(argv[1], "ar") && argc == 2))
        return activate_red();

    else if ((!strcmp(argv[1], "dr") && argc == 2))
        return deactivate_red();

    else if ((!strcmp(argv[1], "br") && argc == 2))
        return blink_red();

    else if ((!strcmp(argv[1], "brf") && argc == 2))
        return blink_red_fast();
    
    else if ((!strcmp(argv[1], "brs") && argc == 2))
        return blink_red_slow();
    
    else if ((!strcmp(argv[1], "brff") && argc == 2))
        return blink_red_super_fast();
        

    else if ((!strcmp(argv[1], "ag") && argc == 2))
        return activate_green();

    else if ((!strcmp(argv[1], "dg") && argc == 2))
        return deactivate_green();

    else if ((!strcmp(argv[1], "bg") && argc == 2))
        return blink_green();

    else if ((!strcmp(argv[1], "bgf") && argc == 2))
        return blink_green_fast();
    
    else if ((!strcmp(argv[1], "bgs") && argc == 2))
        return blink_green_slow();
    
    else if ((!strcmp(argv[1], "bgff") && argc == 2))
        return blink_green_super_fast();
        

    
    else if ((!strcmp(argv[1], "lb") && argc == 2))
        return blink_sync();

    else if ((!strcmp(argv[1], "lbs") && argc == 2))
        return blink_switch();
}
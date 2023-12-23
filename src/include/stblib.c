#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <time.h>

#include "stblib.h"

int msleep(unsigned int ms) {
  return usleep(ms * 1000);
}
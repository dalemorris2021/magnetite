#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#include "lib.h"

int main(void) {
    int32_t x = 3;
    int32_t y = 5;
    int32_t z = add(x, y);
    (void)printf("%d + %d = %d\n", x, y, z);

    return EXIT_SUCCESS;
}

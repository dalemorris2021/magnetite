#include <stdio.h>

#include "chunk.h"
#include "lib.h"

void run(void) {
    Chunk chunk = Chunk_new();
    Chunk_write(&chunk, OP_RETURN);
    Chunk_delete(&chunk);
}

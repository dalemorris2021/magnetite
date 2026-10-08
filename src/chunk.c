#include <stdint.h>

#include "chunk.h"
#include "memory.h"

Chunk Chunk_new(void) {
    return (Chunk){
        .length = 0,
        .capacity = 0,
        .code = NULL,
    };
}

void Chunk_delete(Chunk *chunk) {
    FREE_ARRAY(uint8_t, chunk->code, chunk->capacity);
    chunk->code = NULL;
}

void Chunk_write(Chunk *chunk, uint8_t byte) {
    if (chunk->capacity <= chunk->length) {
        size_t old_capacity = chunk->capacity;
        chunk->capacity = GROW_CAPACITY(old_capacity);
        chunk->code = GROW_ARRAY(uint8_t, chunk->code, old_capacity, chunk->capacity);
    }

    chunk->code[chunk->length] = byte;
    chunk->length++;
}

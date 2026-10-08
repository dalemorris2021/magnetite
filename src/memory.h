#ifndef MAGNETITE_MEMORY_H
#define MAGNETITE_MEMORY_H

#define INIT_CAPACITY 8
#define GROWTH_FACTOR 2

#define GROW_CAPACITY(capacity) ((capacity) < INIT_CAPACITY ? INIT_CAPACITY : (capacity) * GROWTH_FACTOR)

#define GROW_ARRAY(type, pointer, old_capacity, new_capacity)                                                          \
    (type *)reallocate(pointer, sizeof(type) * (old_capacity), sizeof(type) * (new_capacity))

#define FREE_ARRAY(type, pointer, old_capacity) reallocate(pointer, sizeof(type) * (old_capacity), 0)

void *reallocate(void *pointer, size_t old_capacity, size_t new_capacity);

#endif // MAGNETITE_MEMORY_H

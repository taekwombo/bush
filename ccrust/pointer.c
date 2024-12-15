#include <stddef.h>
#include <assert.h>
#include <stdio.h>

void print_ptr(void *ptr) {
    for (int i = 0; i < 8; i++) {
        char *bytes = (char*)ptr;
        printf("%u_", bytes[i]);
    }
}

void convertToBinary(char *name, size_t n) {
    printf("%-12s: ", name);
    printf("%ld", (n >> 63) & 1);
    printf("%ld", (n >> 62) & 1);
    printf("%ld", (n >> 61) & 1);
    printf("%ld", (n >> 60) & 1);
    printf("%ld", (n >> 59) & 1);
    printf("%ld", (n >> 58) & 1);
    printf("%ld", (n >> 57) & 1);
    printf("%ld", (n >> 56) & 1);
    printf(" ");
    printf("%ld", (n >> 55) & 1);
    printf("%ld", (n >> 54) & 1);
    printf("%ld", (n >> 53) & 1);
    printf("%ld", (n >> 52) & 1);
    printf("%ld", (n >> 51) & 1);
    printf("%ld", (n >> 50) & 1);
    printf("%ld", (n >> 49) & 1);
    printf("%ld", (n >> 48) & 1);
    printf(" ");
    printf("%ld", (n >> 47) & 1);
    printf("%ld", (n >> 46) & 1);
    printf("%ld", (n >> 45) & 1);
    printf("%ld", (n >> 44) & 1);
    printf("%ld", (n >> 43) & 1);
    printf("%ld", (n >> 42) & 1);
    printf("%ld", (n >> 41) & 1);
    printf("%ld", (n >> 40) & 1);
    printf(" ");
    printf("%ld", (n >> 39) & 1);
    printf("%ld", (n >> 38) & 1);
    printf("%ld", (n >> 37) & 1);
    printf("%ld", (n >> 36) & 1);
    printf("%ld", (n >> 35) & 1);
    printf("%ld", (n >> 34) & 1);
    printf("%ld", (n >> 33) & 1);
    printf("%ld", (n >> 32) & 1);
    printf(" ");
    printf("%ld", (n >> 31) & 1);
    printf("%ld", (n >> 30) & 1);
    printf("%ld", (n >> 29) & 1);
    printf("%ld", (n >> 28) & 1);
    printf("%ld", (n >> 27) & 1);
    printf("%ld", (n >> 26) & 1);
    printf("%ld", (n >> 25) & 1);
    printf("%ld", (n >> 24) & 1);
    printf(" ");
    printf("%ld", (n >> 23) & 1);
    printf("%ld", (n >> 22) & 1);
    printf("%ld", (n >> 21) & 1);
    printf("%ld", (n >> 20) & 1);
    printf("%ld", (n >> 19) & 1);
    printf("%ld", (n >> 18) & 1);
    printf("%ld", (n >> 17) & 1);
    printf("%ld", (n >> 16) & 1);
    printf(" ");
    printf("%ld", (n >> 15) & 1);
    printf("%ld", (n >> 14) & 1);
    printf("%ld", (n >> 13) & 1);
    printf("%ld", (n >> 12) & 1);
    printf("%ld", (n >> 11) & 1);
    printf("%ld", (n >> 10) & 1);
    printf("%ld", (n >> 9) & 1);
    printf("%ld", (n >> 8) & 1);
    printf(" ");
    printf("%ld", (n >> 7) & 1);
    printf("%ld", (n >> 6) & 1);
    printf("%ld", (n >> 5) & 1);
    printf("%ld", (n >> 4) & 1);
    printf("%ld", (n >> 3) & 1);
    printf("%ld", (n >> 2) & 1);
    printf("%ld", (n >> 1) & 1);
    printf("%ld", (n >> 0) & 1);
    printf("\n");
}

#define binary(name) convertToBinary(#name, name)

void test(int *ptr) {
    size_t one = 1;
    binary(one);
    size_t ptr_size = 48;
    size_t aux = (size_t)1024 << ptr_size;
    binary(aux);

    size_t ptr_mask = ~0;
           ptr_mask = ptr_mask >> (64 - ptr_size);
    binary(ptr_mask);
    size_t aux_mask = (size_t)~0 << ptr_size;
    binary(aux_mask);

    size_t in = (size_t)ptr;
    size_t ptr_part = in & ptr_mask;
    size_t aux_part = aux_mask & aux;
    size_t test_ptr = ptr_part | aux_part;

    binary(ptr_part);
    binary(aux_part);
    binary(test_ptr);
    binary((size_t)ptr);

    printf("Magic number is: %d", *(int*)(test_ptr & ptr_mask));
}

int main() {
    assert(sizeof(size_t) == 8);
    int magic = 777;

    test(&magic);
    return 0;
}

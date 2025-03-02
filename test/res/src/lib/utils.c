#include "utils.h"
#include "bareio.h"

void swap(int *a, int* b) {
    int t = *a;
    *a = *b;
    *b = t;
}

void print_array(int a[], uint n) {
    fputc('[', STDOUT);
    fputi(a[0], STDOUT);
    for (uint i = 1; i < n; i++) {
        fputs(", ", STDOUT);
        fputi(a[i], STDOUT);
    }
    fputs("]\n", STDOUT);
}

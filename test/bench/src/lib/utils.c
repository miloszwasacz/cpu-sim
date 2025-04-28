#include "utils.h"
#include "bareio.h"

void swap(int *a, int* b) {
    int t = *a;
    *a = *b;
    *b = t;
}

void print_array(int a[], uint n) {
    print_char('[');
    print_int(a[0]);
    for (uint i = 1; i < n; i++) {
        print(", ");
        print_int(a[i]);
    }
    print("]\n");
}

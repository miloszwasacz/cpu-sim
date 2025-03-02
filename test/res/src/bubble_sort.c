#include "bareio.h"
#include "utils.h"

void bubble_sort(int *a, uint n) {
    while (n > 1)
    {
        for(uint j = 0; j < n - 1; j++)
        {
            if (a[j] > a[j + 1])
                swap(&a[j], &a[j + 1]);
        }
        n--;
    }
}

int main() {
    int a[] = { 3, 2, 4, 1, 9, 7, 8, 0, 5, 6, 10 };
    uint n = sizeof(a) / sizeof(int);
    print_array(a, n);

    bubble_sort(a, n);
    print_array(a, n);
}

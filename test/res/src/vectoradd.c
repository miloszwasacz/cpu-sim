#include "utils.h"

#define N 10

int main() {
    int a[N] = {10,9,8,7,6,5,4,3,2,1};
    int b[N] = {11,22,33,44,55,66,77,88,99,1010};
    
    for (int i = 0; i < N; i++)
        a[i] += b[i];

    print_array(a, N);
}
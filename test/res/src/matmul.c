#include "bareio.h"

#define N 5

void mat_mul(int a[N][N], int b[N][N], int c[N][N]) {
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            c[i][j] = 0;
            for (int k = 0; k < N; k++)
                c[i][j] += a[i][k] * b[k][j];
        }
    }
}

int main() {
    int a[N][N] = {{1,2,3,4,5}, {6,7,8,9,10}, {11,12,13,14,15},{16,17,18,19,20},{21,22,23,24,25}};
    int b[N][N] = {{25,24,23,22,21},{20,19,18,17,16},{15,14,13,12,11},{10,9,8,7,6},{5,4,3,2,1}};
    int c[N][N];
    mat_mul(a, b, c); 
    int x = c[0][0];
    print_int(x);
}
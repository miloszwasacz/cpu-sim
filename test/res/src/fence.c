#include "bareio.h"

#define N 5

int main() {
    int a[N] = {1, 2, 3, 4, 5};
    const int l = N - 1;
    for (int i = 0; i < N; i++) {
        asm("ebreak");
        int x = 0, y = 0;
        asm(
            "nop\n"    
            "nop\n"    
            "nop\n"    
            "nop\n"    
            "nop\n"    
            "nop\n"    
            "nop\n"    
            "nop\n"    
            "nop\n"    
            "fence"
        );
        x = a[i];
        if (i < l) {
            asm("fence");
            y = a[i];
        }
        print_int(x + y);
    }
}
#include "bareio.h"

unsigned int factorial(unsigned int k) {
    if (k <= 1)
        return 1;
    
    return k * factorial(k - 1);
}

int main() {
    unsigned int fac = factorial(12);
    fputi(fac, STDOUT);
    putchar('\n');
}

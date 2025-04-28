unsigned int factorial(unsigned int k) {
    if (k <= 1)
        return 1;
    
    return k * factorial(k - 1);
}

int main() {
    volatile unsigned int fac = factorial(12);
}

 #include<stdio.h>

unsigned int factorial(unsigned int k) {
    if (k <= 1)
        return 1;
    
    return k * factorial(k - 1);
}

int main() {
    unsigned int fac = factorial(12);
    
    const int LEN = 12;
    char chars[LEN];
    chars[0] = '0'; chars[1] = 'x'; chars[LEN - 2] = '\n'; chars[LEN - 1] = '\0';
    unsigned int f = fac;
    for (unsigned int i = LEN - 3, f = fac; f > 0; i--) {
        unsigned int masked = f & 0xf;
        chars[i] = masked < 10 ? masked + '0' : masked - 10 + 'a';
        f = f >> 4;
    }

    _write(1, chars, LEN);
}
#include<stdlib.h>
#include "bareio.h"

int fprint_char(int c, int fd) {
    unsigned char buf[1];
    buf[0] = (unsigned char)c;
    if (write(fd, buf, 1) != -1) {
        return (int)buf[0];
    } else {
        return EOF;    
    }
}

int print_char(int c) {
    return fprint_char(c, STDOUT);
}

int fprint_int(int i, int fd) {
    int l = i;
    unsigned int len = i == 0 ? 2 : 1;
    while (l > 0) {
        len++;
        l /= 10;
    }
    char *text = malloc(len * sizeof(char));
    text[len - 1] = '\0';
    for (int j = len - 2; j >= 0; j--) {
        int k = i % 10;
        text[j] = k + '0';
        i /= 10;
    }

    if (fprint_str(text, fd) == EOF)
        return EOF;

    return 1;
}

int fprint_int_hex(int i, int fd) {
    const unsigned int len = 2 + 8 + 1;
    char *text = malloc(len * sizeof(char));
    text[0] = '0';
    text[1] = 'x';
    text[len - 1] = '\0';
    for (int j = len - 2; j > 1; j--) {
        int k = i & 0xf;
        text[j] = k < 0xa ? k + '0' : k - 0xa + 'a';
        i = i >> 4;
    }
    
    if (fprint_str(text, fd) == EOF)
        return EOF;
    
    return 1;
}

int fprint_int_bin(int i, int fd) {
    const unsigned int len = 2 + 32 + 1;
    char *text = malloc(len * sizeof(char));
    text[0] = '0';
    text[1] = 'b';
    text[len - 1] = '\0';
    for (int j = len - 2; j > 1; j--) {
        text[j] = (i & 1) + '0';
        i = i >> 1;
    }
    
    if (fprint_str(text, fd) == EOF)
        return EOF;
    
    return 1;
}

int print_int(int i) {
    return fprint_int(i, STDOUT);
}

int fprint_str(const char *s, int fd) {
    for (unsigned int i = 0; s[i] != '\0'; i++) {
        if (fprint_char(s[i], fd) == EOF)
            return EOF;
    }

    return 1;
}

int print(const char *s) {
    if (fprint_str(s, STDOUT) == EOF)
        return EOF;
    
    return 1;
}

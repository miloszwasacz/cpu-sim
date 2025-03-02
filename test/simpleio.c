#include<stdlib.h>
#include "simpleio.h"

int fputc(int c, int fd) {
    unsigned char buf[1];
    buf[0] = (unsigned char)c;
    if (write(fd, buf, 1) != -1) {
        return (int)buf[0];
    } else {
        return EOF;    
    }
}

int putchar(int c) {
    return fputc(c, STDOUT);
}

//TODO Implement this when division and remainder are supported
//void fputi(int i, int fd) {
//
//}

int fputix(int i, int fd) {
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
    
    if (fputs(text, fd) == EOF)
        return EOF;
    
    return 1;
}

int fputib(int i, int fd) {
    const unsigned int len = 2 + 32 + 1;
    char *text = malloc(len * sizeof(char));
    text[0] = '0';
    text[1] = 'b';
    text[len - 1] = '\0';
    for (int j = len - 2; j > 1; j--) {
        text[j] = (i & 1) + '0';
        i = i >> 1;
    }
    
    if (fputs(text, fd) == EOF)
        return EOF;
    
    return 1;
}

int fputs(const char *s, int fd) {
    for (unsigned int i = 0; s[i] != '\0'; i++) {
        if (fputc(s[i], fd) == EOF)
            return EOF;
    }
    return 1;
}

int puts(const char *s) {
    if (fputs(s, STDOUT) == EOF)
        return EOF;
    
    if (fputc('\n', STDOUT) == EOF)
        return EOF;
    
    return 1;
}

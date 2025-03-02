#ifndef BARE_METAL_IO_
#define BARE_METAL_IO_

#pragma GCC diagnostic push 
#pragma GCC diagnostic ignored "-Wbuiltin-declaration-mismatch"

#define STDIN 0
#define STDOUT 1
#define STDERR 2

#define EOF -1

int write(int fd, char *buf, int count);

int fputc(int c, int fd);
int putchar(int c);

int fputi(int i, int fd);
int fputix(int i, int fd);
int fputib(int i, int fd);

int fputs(const char *s, int fd);
int puts(const char *s);

#pragma GCC diagnostic pop
#endif

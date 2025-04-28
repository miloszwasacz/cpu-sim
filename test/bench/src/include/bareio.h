#ifndef BARE_METAL_IO_
#define BARE_METAL_IO_

#define STDIN 0
#define STDOUT 1
#define STDERR 2

#define EOF -1

int write(int fd, char *buf, int count);

int fprint_char(int c, int fd);
int print_char(int c);

int fprint_int(int i, int fd);
int fprint_int_hex(int i, int fd);
int fprint_int_bin(int i, int fd);
int print_int(int i);

int fprint_str(const char *s, int fd);
int print(const char *s);

#endif

#ifndef SYSTEM_CALLS_
#define SYSTEM_CALLS_

#define EXIT_CODE 93

__attribute__((noreturn)) static inline void exit(int code) {
    asm volatile("mv a0, %[exitcode]\n\t"
                 "li a7, %[syscode]\n\t"
                 "ecall"
                 :
                 : [exitcode] "r" (code), [syscode] "i" (EXIT_CODE)
                 : "a0", "a7");
    __builtin_unreachable();
}

#endif
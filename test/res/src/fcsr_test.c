#include "bareio.h"

#define TEST_COUNT 12

int test_csr_value(const char * test_name, int expected, int actual) {
    print(test_name);
    if (expected == actual) {
        print("[Passed]\n");
        return 0;
    }    

    print("[Failed]\n  expected: ");
    fprint_int_bin(expected, STDOUT);
    print("\n  got:      ");
    fprint_int_bin(actual, STDOUT);
    print_char('\n');
    return 1;
}

int main() {
    const int FLAGS = 0x11;               // 0b10001
    const int RM    = 0x2;                // 0b010
    const int CSR   = (RM << 5) | FLAGS;  // 0b010_10001
    const int RM_MASK    = 0x7;           // 0b111
    const int FLAGS_MASK = 0x1f;          // 0b11111
    
    volatile int csr0, flags0, rm0;
    volatile int csr1, flags1, rm1;
    volatile int csr2, flags2, rm2;
    volatile int csr3, flags3, rm3;

    // From immediates
    asm(
        "ebreak\n"
        "frcsr    %[csr0]\n"
        "fsrmi    %[rm0], %[rm_imm]\n"
        "fsflagsi %[flags0], %[flags_imm]\n"
        "frrm     %[rm1]\n"
        "frflags  %[flags1]\n"
        "frcsr    %[csr1]"
        : [rm0] "=r" (rm0), [flags0] "=r" (flags0), [csr0] "=r" (csr0), 
          [rm1] "=r" (rm1), [flags1] "=r" (flags1), [csr1] "=r" (csr1)
        : [rm_imm] "i" (RM), [flags_imm] "i" (FLAGS)
        : "cc"
    );
    
    // From registers
    asm(
        "ebreak\n"
        "fscsr   %[csr2], %[new_csr]\n"
        "fsrm    %[rm2], %[rm_reg]\n"
        "fsflags %[flags2], %[flags_reg]\n"
        "frrm     %[rm3]\n"
        "frflags  %[flags3]\n"
        "frcsr   %[csr3]"
        : [rm2] "=r" (rm2), [flags2] "=r" (flags2), [csr2] "=r" (csr2),
          [rm3] "=r" (rm3), [flags3] "=r" (flags3), [csr3] "=r" (csr3)
        : [new_csr] "r" (~CSR), [rm_reg] "r" (RM), [flags_reg] "r" (FLAGS)
        : "cc"
    );


    int tests[TEST_COUNT];
    tests[0] = test_csr_value("csr_def...     ", 0, csr0);
    tests[1] = test_csr_value("rm_def...      ", 0, rm0);
    tests[2] = test_csr_value("flags_def...   ", 0, flags0);

    tests[3] = test_csr_value("csr_imm...     ", CSR, csr1);
    tests[4] = test_csr_value("rm_imm...      ", RM, rm1);
    tests[5] = test_csr_value("flags_imm...   ", FLAGS, flags1);

    tests[6] = test_csr_value("csr_swap...    ", CSR, csr2);
    tests[7] = test_csr_value("rm_swap...     ", ~RM & RM_MASK, rm2);
    tests[8] = test_csr_value("flags_swap...  ", ~FLAGS & FLAGS_MASK, flags2);

    tests[9] = test_csr_value("csr_reg...     ", CSR, csr3);
    tests[10] = test_csr_value("rm_reg...      ", RM, rm3);
    tests[11] = test_csr_value("flags_reg...   ", FLAGS, flags3);


    int result = 0;
    for (int i = 0; i < TEST_COUNT; i++) {
        result |= tests[i] << i;
    }
    return result;
}
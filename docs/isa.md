# Instruction Set Architecture

This is an ISA used by the CPU simulator, based on Arm A-profile A64 ISA.

## Instructions

### Data Processing -- Immediate

- [ADD (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ADD--immediate---Add-immediate-value-?lang=en)
- [ADDS (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ADDS--immediate---Add-immediate-value--setting-flags-)
- [SUB (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/SUB--immediate---Subtract-immediate-value-?lang=en)
- [SUBS (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/SUBS--immediate---Subtract-immediate-value--setting-flags-)
- [SMAX (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/SMAX--immediate---Signed-maximum--immediate--?lang=en)
- [UMAX (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/UMAX--immediate---Unsigned-maximum--immediate--?lang=en)
- [SMIN (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/SMIN--immediate---Signed-minimum--immediate--?lang=en)
- [UMIN (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/UMIN--immediate---Unsigned-minimum--immediate--?lang=en)
- [AND (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/AND--immediate---Bitwise-AND--immediate--?lang=en)
- [ORR (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ORR--immediate---Bitwise-OR--immediate--?lang=en)
- [EOR (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/EOR--immediate---Bitwise-exclusive-OR--immediate--?lang=en)
- [ANDS (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ANDS--immediate---Bitwise-AND--immediate---setting-flags-?lang=en)
- [MOVZ](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/MOVZ--Move-wide-with-zero-?lang=en)

### Branches and System Instructions

- [B.cond](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/B-cond--Branch-conditionally-)
- [NOP](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/NOP--No-operation-?lang=en)
- [BR](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/BR--Branch-to-register-?lang=en)
- [RET](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/RET--Return-from-subroutine-?lang=en)
- [B](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/B--Branch-?lang=en)

> [!NOTE]
> The simulator halts on the first encountered `RET` instruction.

### Data Processing -- Register

- [LSLV](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LSLV--Logical-shift-left-variable-?lang=en)
- [LSRV](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LSRV--Logical-shift-right-variable-?lang=en)
- [ASRV](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ASRV--Arithmetic-shift-right-variable-?lang=en)
- [RORV](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/RORV--Rotate-right-variable-?lang=en)
- [SMAX (register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/SMAX--register---Signed-maximum--register--?lang=en)
- [UMAX (register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/UMAX--register---Unsigned-maximum--register--?lang=en)
- [SMIN (register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/SMIN--register---Signed-minimum--register--?lang=en)
- [UMIN (register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/UMIN--register---Unsigned-minimum--register--?lang=en)
- [ABS](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ABS--Absolute-value-?lang=en)
- [AND (shifted register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/AND--shifted-register---Bitwise-AND--shifted-register--?lang=en)
- [ORR (shifted register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ORR--shifted-register---Bitwise-OR--shifted-register--?lang=en)
- [EOR (shifted register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/EOR--shifted-register---Bitwise-exclusive-OR--shifted-register--?lang=en)
- [ANDS (shifted register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ANDS--shifted-register---Bitwise-AND--shifted-register---setting-flags-?lang=en)
- [ADD (shifted register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ADD--shifted-register---Add-optionally-shifted-register-?lang=en)
- [ADDS (shifted register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/ADDS--shifted-register---Add-optionally-shifted-register--setting-flags-)
- [SUB (shifted register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/SUB--shifted-register---Subtract-optionally-shifted-register-?lang=en)
- [SUBS (shifted register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/SUBS--shifted-register---Subtract-optionally-shifted-register--setting-flags-)
- [MADD](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/MADD--Multiply-add-?lang=en)

### Loads and Stores

> [!NOTE]
> All the load and store instructions below that operate on immediate values 
> currently default to unsigned offset addressing, 
> i.e. pre- and post-indexing is not supported. 

#### Loads

- [LDR (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDR--immediate---Load-register--immediate--?lang=en)
- [LDR (register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDR--register---Load-register--register--?lang=en)
- [LDRB (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDRB--immediate---Load-register-byte--immediate--)
- [LDRB (register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDRB--register---Load-register-byte--register--)
- [LDRH (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDRH--immediate---Load-register-halfword--immediate--)
- [LDRH (register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDRH--register---Load-register-halfword--register--)
- [LDRSB (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDRSB--immediate---Load-register-signed-byte--immediate--)
- [LDRSB (register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDRSB--register---Load-register-signed-byte--register--)
- [LDRSH (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDRSH--immediate---Load-register-signed-halfword--immediate--)
- [LDRSH (register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDRSH--register---Load-register-signed-halfword--register--)
- [LDRSW (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDRSW--immediate---Load-register-signed-word--immediate--)
- [LDRSW (register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDRSW--register---Load-register-signed-word--register--)

#### Stores

- [STR (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/STR--immediate---Store-register--immediate--?lang=en)
- [STR (register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/STR--register---Store-register--register--?lang=en)
- [STRB (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/STRB--immediate---Store-register-byte--immediate--)
- [STRB (register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/STRB--register---Store-register-byte--register--)
- [STRH (immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/STRH--immediate---Store-register-halfword--immediate--)
- [STRH (register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/STRH--register---Store-register-halfword--register--)

### Aliases

- [MOV (wide immediate)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/MOV--wide-immediate---Move-wide-immediate-value--an-alias-of-MOVZ-?lang=en)
- [MOV (register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/MOV--register---Move-register-value--an-alias-of-ORR--shifted-register--?lang=en)
- [NEG (shifted register)](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/NEG--shifted-register---Negate--shifted-register---an-alias-of-SUB--shifted-register--?lang=en)
- [MUL](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/MUL--Multiply--an-alias-of-MADD-?lang=en)
- [MSUB](https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/MSUB--Multiply-subtract-?lang=en)
  
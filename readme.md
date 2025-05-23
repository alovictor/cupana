# Cupana Machine and Emulator

uma cpu de 16bits
64kb de memória
16 registradores u16:
    - reg 0: acc
    - reg 1 .. reg15: general purpuse

## Flags

| NAME      | POSITION    |
|-----------|-------------|
| Zero      | 0b0000_000x |
| Carry     | 0b0000_00x0 |
| Negative  | 0b0000_0x00 |
| Overflow  | 0b0000_x000 |
| Interrupt | 0b000x_0000 |
| Halt      | 0bx000_0000 |

## Instruções

| #  | INSTRUCTION | OPCODE | DESCRIPTION                                    | USAGE        | TYPE          |
|----|-------------|--------|------------------------------------------------|--------------|---------------|
| 1  | NOP         | 0x00   | No Operation                                   | NOP          | Control       |
| 2  | HLT         | 0x01   | Halt                                           | HLT          | Control       |
| 3  | MOV         | 0x10   | Copy from Register to Register                 | MOV Reg Reg  | Data Transfer |
| 4  | MOV         | 0x11   | Load Register with Literal                     | MOV Reg Lit  | Data Transfer |
| 5  | MOV         | 0X12   | Load Register with value from Memory Direct    | MOV Reg Mem  | Data Transfer |
| 6  | MOV         | 0X13   | Load Register with value from Memory Indirect  | MOV Reg Reg* | Data Transfer |
| 7  | MOV         | 0x14   | Stores value from Register                     | MOV Mem Reg  | Data Transfer |
| 8  | MOV         | 0x15   | Stores value from Register Indirect            | MOV Reg* Reg | Data Transfer |
| 9  | ADD         | 0x20   | Add two Registers                              | ADD Reg Reg  | Arithmetic    |
| 10 | ADD         | 0x21   | Add Register with Literal                      | ADD Reg Lit  | Arithmetic    |
| 11 | SUB         | 0x22   | Subtract two Registers                         | SUB Reg Reg  | Arithmetic    |
| 12 | SUB         | 0x23   | Subtract Register with Literal                 | SUB Reg Lit  | Arithmetic    |
| 13 | SUB         | 0x24   | Subtract Literal with Register                 | SUB Lit Reg  | Arithmetic    |
| 14 | MUL         | 0x25   | Multiply two Registers                         | MUL Reg Reg  | Arithmetic    |
| 15 | MUL         | 0x26   | Multiply Register with Literal                 | MUL Reg Lit  | Arithmetic    |
| 16 | DIV         | 0x27   | Divide two Registers                           | DIV Reg Reg  | Arithmetic    |
| 17 | DIV         | 0x28   | Divide Register with Literal                   | DIV Reg Lit  | Arithmetic    |
| 18 | DIV         | 0x29   | Divide Literal with Register                   | DIV Lit Reg  | Arithmetic    |
| 19 | MOD         | 0x2A   | Modulo two Registers                           | MOD Reg Reg  | Arithmetic    |
| 20 | MOD         | 0x2B   | Modulo Register with Literal                   | MOD Reg Lit  | Arithmetic    |
| 21 | MOD         | 0x2C   | Modulo Literal with Register                   | MOD Lit Reg  | Arithmetic    |
| 22 | INC         | 0x2D   | Increment a Register value                     | INC Reg      | Arithmetic    |
| 23 | DEC         | 0x2E   | Decrement a Register value                     | DEC Reg      | Arithmetic    |
| 24 | AND         | 0x30   | Bitwise AND between two registers              | AND Reg Reg  | Arithmetic    |
| 25 | OR          | 0x31   | Bitwise OR between two registers               | OR  Reg Reg  | Arithmetic    |
| 26 | XOR         | 0x32   | Bitwise XOR between two registers              | XOR Reg Reg  | Arithmetic    |
| 27 | NOT         | 0x33   | Bitwise NOT between two registers              | NOT Reg      | Arithmetic    |
| 28 | CMP         | 0x40   | Compare two registers                          | CMP Reg Reg  | Comparison    |
| 29 | CMP         | 0x41   | Compare a Register with Literal                | CMP Reg Lit  | Comparison    |
| 30 | JMP         | 0x50   | Unconditional Jump to Literal memory address   | JMP Lit      | Control Flow  |
| 31 | JMP         | 0x51   | Unconditional Jump to Register pointer         | JMP Reg      | Control Flow  |
| 32 | JZ          | 0x52   | Jump if Zero to Literal memory address         | JZ  Lit      | Control Flow  |
| 33 | JZ          | 0x53   | Jump if Zero to Register pointer               | JZ  Reg      | Control Flow  |
| 34 | JNZ         | 0x54   | Jump if Not Zero to Literal memory address     | JNZ Lit      | Control Flow  |
| 35 | JNZ         | 0x55   | Jump if Not Zero to Register pointer           | JNZ Reg      | Control Flow  |
| 36 | JN          | 0x56   | Jump if Negative to Literal memory address     | JN  Lit      | Control Flow  |
| 37 | JN          | 0x57   | Jump if Negative to Register pointer           | JN  Reg      | Control Flow  |
| 38 | JNN         | 0x58   | Jump if Not Negative to Literal memory address | JNN Lit      | Control Flow  |
| 39 | JNN         | 0x59   | Jump if Not Negative to Register pointer       | JNN Reg      | Control Flow  |
| 40 | JC          | 0x5A   | Jump if Carry to Literal memory address        | JC  Lit      | Control Flow  |
| 41 | JC          | 0x5B   | Jump if Carry to Register pointer              | JC  Reg      | Control Flow  |
| 42 | JNC         | 0x5C   | Jump if Not Carry to Literal memory address    | JNC Lit      | Control Flow  |
| 43 | JNC         | 0x5D   | Jump if Not Carry to Register pointer          | JNC Reg      | Control Flow  |
| 44 | CALL        | 0x60   | Subroutine Call                                | CALL Lit     | Control Flow  |
| 45 | RET         | 0x61   | Subroutine Return                              | RET          | Control Flow  |

where:

| Arg       | Format           |
|-----------|------------------|
| LITERAL   | $decimal or #hex |
| REGISTER  | R(id)            |
| REGISTER* | R(id)*           |

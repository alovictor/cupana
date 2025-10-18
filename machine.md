# Cupana System

- 16bits
- 64kb de memória
- 14 registradores de uso geral: R0 - R13
- 2 registradores especiais:
    - R14: PC
    - R15: SP

## Address Space

| Type    | Range               | Size |
|---------|---------------------|------|
| ROM     | `0x0000` - `0x7FFF` | 32kb |
| RAM     | `0x8000` - `0xEFFF` | 28kb |
| Devices | `0xF000` - `0xFFFF` | 4kb  |

## Flags

| NAME               | POSITION      |
|--------------------|---------------|
| Zero               | `0b0000_0001` |
| Carry              | `0b0000_0010` |
| Negative           | `0b0000_0100` |
| Interrupt Disabled | `0b0000_1000` |
| Interrupt Pending  | `0b0001_0000` |
| Halt               | `0b1000_0000` |

## Instruções

| #  | OPCODE | HEX    | DESCRIPTION                 | TYPE             |
|----|--------|--------|-----------------------------|------------------|
| 1  | NOP    | `0x00` | No Operation                | Control          |
| 2  | HLT    | `0x01` | Halt execution              | Control          |
| 3  | MOV    | `0x02` | Move data                   | Data Transfer    |
| 4  | PHR    | `0x03` | Push register to stack      | Data Transfer    |
| 5  | PLR    | `0x04` | Pull from stack to register | Data Transfer    |
| 6  | ADD    | `0x05` | Add                         | Arithmetic       |
| 7  | SUB    | `0x06` | Subtract                    | Arithmetic       |
| 8  | MUL    | `0x07` | Multiply                    | Arithmetic       |
| 9  | DIV    | `0x08` | Divide                      | Arithmetic       |
| 10 | MOD    | `0x09` | Modulo                      | Arithmetic       |
| 11 | INC    | `0x0A` | Increment                   | Arithmetic       |
| 12 | DEC    | `0x0B` | Decrement                   | Arithmetic       |
| 13 | AND    | `0x0C` | Bitwise AND                 | Logic            |
| 14 | OR     | `0x0D` | Bitwise OR                  | Logic            |
| 15 | SHL    | `0x0E` | Shift Left                  | Logic            |
| 16 | SHR    | `0x0F` | Shift Right                 | Logic            |
| 17 | NOT    | `0x10` | Bitwise NOT                 | Logic            |
| 18 | CMP    | `0x11` | Compare                     | Comparison       |
| 19 | JMP    | `0x12` | Unconditional jump          | Control Flow     |
| 20 | JZ     | `0x13` | Jump if Zero                | Control Flow     |
| 21 | JNZ    | `0x14` | Jump if Not Zero            | Control Flow     |
| 22 | JN     | `0x15` | Jump if Negative            | Control Flow     |
| 23 | JNN    | `0x16` | Jump if Not Negative        | Control Flow     |
| 24 | JC     | `0x17` | Jump if Carry               | Control Flow     |
| 25 | JNC    | `0x18` | Jump if Not Carry           | Control Flow     |
| 26 | JSB    | `0x19` | Jump to Subroutine          | Control Flow     |
| 27 | RSB    | `0x1A` | Return from Subroutine      | Control Flow     |
| 28 | CLI    | `0x1B` | Clear Interrupt flag        | Interrupt Handle |
| 29 | SEI    | `0x1C` | Set Interrupt flag          | Interrupt Handle |
| 30 | RSI    | `0x1D` | Return from Interrupt       | Interrupt Handle |

As instruções Aritméticas e Lógicas sempre retornam o resultado no primeiro registrador argumento (Reg)

### Formato da Instrução

| Tamanho | Instruções                                                                                 |
|---------|--------------------------------------------------------------------------------------------|
| 8 bits  | NOP, HLT, RSB, CLI, SEI, RSI                                                               |
| 16 bits | PHR, PLR, INC, DEC, NOT, JMP, JZ, JNZ, JN, JNN, JC, JNC, JSB                               |
| 24 bits | MOV, ADD, SUB, MUL, DIV, MOD, AND, OR, SHL, SHR, CMP,  JMP, JZ, JNZ, JN, JNN, JC, JNC, JSB |
| 32 bits | MOV, ADD, SUB, MUL, DIV, MOD, AND, OR, SHL, SHR, CMP                                       |

| Campo        | Bits  | Tamanho                 | Descrição                                                                                        |
|--------------|-------|-------------------------|--------------------------------------------------------------------------------------------------|
| Opcode       | 31-27 | 5 bits                  | Código da operação a ser executada.                                                              |
| B (Byte)     | 26    | 1 bit                   | `0`: Operação com word (16 bits); `1`: Operação com byte (8 bits).                               |
| L (Literal)  | 25    | 1 bit                   | `0`: ARG 2 é um registrador; `1`: ARG 2 é um valor literal.                                      |
| I (Indirect) | 24    | 1 bit                   | `0`: ARG 1 é o valor do registrador; `1`: ARG 1 é um endereço de memória contido no registrador. |
| ARG 1        | 23-16 | 8 bits (Reg) ou 16 bits | Primeiro argumento, geralmente o registrador de destino.                                         |
| ARG 2        | 15-0  | 16 bits ou 8 bits (Reg) | Segundo argumento, pode ser um registrador ou um valor literal de 16 bits.                       |

| OPCODE | B | L | I | ARG 1 | ARG 2 |
|--------|---|---|---|-------|-------|
| NOP    | 0 | 0 | 0 | -     | -     |
| HLT    | 0 | 0 | 0 | -     | -     |
| MOV    | 0 | 0 | 0 | Reg   | Reg   |
| MOV    | 0 | 1 | 0 | Reg   | Lit   |
| MOV    | 1 | 0 | 0 | Reg   | Reg   |
| MOV    | 1 | 1 | 0 | Reg   | Lit   |
| MOV    | 0 | 0 | 1 | Reg*  | Reg   |
| MOV    | 0 | 1 | 1 | Reg*  | Lit   |
| MOV    | 1 | 0 | 1 | Reg*  | Reg   |
| MOV    | 1 | 1 | 1 | Reg*  | Lit   |
| PHR    | 0 | 0 | 0 | Reg   | -     |
| PLR    | 0 | 0 | 0 | Reg   | -     |
| ADD    | 0 | 0 | 0 | Reg   | Reg   |
| ADD    | 0 | 1 | 0 | Reg   | Lit   |
| ADD    | 1 | 0 | 0 | Reg   | Reg   |
| ADD    | 1 | 1 | 0 | Reg   | Lit   |
| SUB    | 0 | 0 | 0 | Reg   | Reg   |
| SUB    | 0 | 1 | 0 | Reg   | Lit   |
| SUB    | 1 | 0 | 0 | Reg   | Reg   |
| SUB    | 1 | 1 | 0 | Reg   | Lit   |
| MUL    | 0 | 0 | 0 | Reg   | Reg   |
| MUL    | 0 | 1 | 0 | Reg   | Lit   |
| MUL    | 1 | 0 | 0 | Reg   | Reg   |
| MUL    | 1 | 1 | 0 | Reg   | Lit   |
| DIV    | 0 | 0 | 0 | Reg   | Reg   |
| DIV    | 0 | 1 | 0 | Reg   | Lit   |
| DIV    | 1 | 0 | 0 | Reg   | Reg   |
| DIV    | 1 | 1 | 0 | Reg   | Lit   |
| MOD    | 0 | 0 | 0 | Reg   | Reg   |
| MOD    | 0 | 1 | 0 | Reg   | Lit   |
| MOD    | 1 | 0 | 0 | Reg   | Reg   |
| MOD    | 1 | 1 | 0 | Reg   | Lit   |
| INC    | 0 | 0 | 0 | Reg   | -     |
| INC    | 1 | 0 | 0 | Reg   | -     |
| DEC    | 0 | 0 | 0 | Reg   | -     |
| DEC    | 1 | 0 | 0 | Reg   | -     |
| AND    | 0 | 0 | 0 | Reg   | Reg   |
| AND    | 0 | 1 | 0 | Reg   | Lit   |
| AND    | 1 | 0 | 0 | Reg   | Reg   |
| AND    | 1 | 1 | 0 | Reg   | Lit   |
| OR     | 0 | 0 | 0 | Reg   | Reg   |
| OR     | 0 | 1 | 0 | Reg   | Lit   |
| OR     | 1 | 0 | 0 | Reg   | Reg   |
| OR     | 1 | 1 | 0 | Reg   | Lit   |
| SHL    | 0 | 0 | 0 | Reg   | Reg   |
| SHL    | 0 | 1 | 0 | Reg   | Lit   |
| SHR    | 0 | 0 | 0 | Reg   | Reg   |
| SHR    | 0 | 1 | 0 | Reg   | Lit   |
| NOT    | 0 | 0 | 0 | Reg   | -     |
| NOT    | 1 | 0 | 0 | Reg   | -     |
| CMP    | 0 | 0 | 0 | Reg   | Reg   |
| CMP    | 0 | 1 | 0 | Reg   | Lit   |
| CMP    | 1 | 0 | 0 | Reg   | Reg   |
| CMP    | 1 | 1 | 0 | Reg   | Lit   |
| JMP    | 0 | 0 | 0 | Reg   | -     |
| JMP    | 0 | 1 | 0 | Lit   | -     |
| JZ     | 0 | 0 | 0 | Reg   | -     |
| JZ     | 0 | 1 | 0 | Lit   | -     |
| JNZ    | 0 | 0 | 0 | Reg   | -     |
| JNZ    | 0 | 1 | 0 | Lit   | -     |
| JN     | 0 | 0 | 0 | Reg   | -     |
| JN     | 0 | 1 | 0 | Lit   | -     |
| JNN    | 0 | 0 | 0 | Reg   | -     |
| JNN    | 0 | 1 | 0 | Lit   | -     |
| JC     | 0 | 0 | 0 | Reg   | -     |
| JC     | 0 | 1 | 0 | Lit   | -     |
| JNC    | 0 | 0 | 0 | Reg   | -     |
| JNC    | 0 | 1 | 0 | Lit   | -     |
| JSB    | 0 | 1 | 0 | Lit   | -     |
| RSB    | 0 | 0 | 0 | -     | -     |
| CLI    | 0 | 0 | 0 | -     | -     |
| SEI    | 0 | 0 | 0 | -     | -     |
| RSI    | 0 | 0 | 0 | -     | -     |

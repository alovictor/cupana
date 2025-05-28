# Cupana Machine and Emulator

uma cpu de 16bits
64kb de memória
16 registradores u16: R0 - R15

## Address Space

| Type    | Range           | Size |
|---------|-----------------|------|
| ROM     | 0x0000 - 0x7FFF | 32kb |
| RAM     | 0x8000 - 0xDFFF | 28kb |
| Stack   | 0xE000 - 0xEFFF | 4kb  |
| Devices | 0xF000 - 0xFFFF | 4kb  |

## Flags

| NAME               | POSITION    |
|--------------------|-------------|
| Zero               | 0b0000_000x |
| Carry              | 0b0000_00x0 |
| Negative           | 0b0000_0x00 |
| Interrupt Disabled | 0b0000_x000 |
| Interrupt Pending  | 0b000x_0000 |
| Halt               | 0bx000_0000 |

## Instruções

As instruções Aritméticas e Lógicas sempre retornam o resultado no primeiro registrador argumento (Reg)

| #  | INSTRUCTION | OPCODE | DESCRIPTION                                    | USAGE        | TYPE             |
|----|-------------|--------|------------------------------------------------|--------------|------------------|
| 1  | NOP         | 0x00   | No Operation                                   | NOP          | Control          |
| 2  | HLT         | 0x01   | Halt                                           | HLT          | Control          |
| 3  | MOV         | 0x10   | Copy from Register to Register                 | MOV Reg Reg  | Data Transfer    |
| 4  | MOV         | 0x11   | Load Register with Literal                     | MOV Reg Lit  | Data Transfer    |
| 5  | MOV         | 0X12   | Load Register with value from Memory Indirect  | MOV Reg Reg* | Data Transfer    |
| 6  | MOV         | 0x13   | Stores value from Register                     | MOV Mem Reg  | Data Transfer    |
| 7  | MOV         | 0x14   | Stores value from Literal                      | MOV Mem Lit  | Data Transfer    |
| 8  | MOV         | 0x15   | Stores value from Register Indirect            | MOV Reg* Reg | Data Transfer    |
| 9  | MOV         | 0x16   | Stores value from Literal Indirect             | MOV Reg* Lit | Data Transfer    |
| 10 | PHR         | 0x17   | Push register value to stack                   | PHR Reg      | Data Transfer    |
| 11 | PLR         | 0x18   | Pull value from stack                          | PLR Reg      | Data Transfer    |
| 12 | ADD         | 0x20   | Add two Registers                              | ADD Reg Reg  | Arithmetic       |
| 13 | ADD         | 0x21   | Add Register with Literal                      | ADD Reg Lit  | Arithmetic       |
| 14 | SUB         | 0x22   | Subtract two Registers                         | SUB Reg Reg  | Arithmetic       |
| 15 | SUB         | 0x23   | Subtract Register with Literal                 | SUB Reg Lit  | Arithmetic       |
| 16 | MUL         | 0x24   | Multiply two Registers                         | MUL Reg Reg  | Arithmetic       |
| 17 | MUL         | 0x25   | Multiply Register with Literal                 | MUL Reg Lit  | Arithmetic       |
| 18 | DIV         | 0x26   | Divide two Registers                           | DIV Reg Reg  | Arithmetic       |
| 19 | DIV         | 0x27   | Divide Register with Literal                   | DIV Reg Lit  | Arithmetic       |
| 20 | MOD         | 0x28   | Modulo two Registers                           | MOD Reg Reg  | Arithmetic       |
| 21 | MOD         | 0x29   | Modulo Register with Literal                   | MOD Reg Lit  | Arithmetic       |
| 22 | INC         | 0x2A   | Increment a Register value                     | INC Reg      | Arithmetic       |
| 23 | DEC         | 0x2B   | Decrement a Register value                     | DEC Reg      | Arithmetic       |
| 24 | AND         | 0x30   | Bitwise AND between two registers              | AND Reg Reg  | Logic            |
| 25 | AND         | 0x31   | Bitwise AND between register and literal       | AND Reg Lit  | Logic            |
| 26 | OR          | 0x32   | Bitwise OR between two registers               | OR  Reg Reg  | Logic            |
| 27 | OR          | 0x33   | Bitwise OR between register and literal        | OR  Reg Lit  | Logic            |
| 28 | XOR         | 0x34   | Bitwise XOR between two registers              | XOR Reg Reg  | Logic            |
| 29 | XOR         | 0x35   | Bitwise XOR between register and literal       | XOR Reg Lit  | Logic            |
| 30 | NOT         | 0x36   | Bitwise NOT a register                         | NOT Reg      | Logic            |
| 31 | CMP         | 0x40   | Compare two registers                          | CMP Reg Reg  | Comparison       |
| 32 | CMP         | 0x41   | Compare a Register with Literal                | CMP Reg Lit  | Comparison       |
| 33 | JMP         | 0x50   | Unconditional Jump to Literal memory address   | JMP Lit      | Control Flow     |
| 34 | JMP         | 0x51   | Unconditional Jump to Register pointer         | JMP Reg      | Control Flow     |
| 35 | JZ          | 0x52   | Jump if Zero to Literal memory address         | JZ  Lit      | Control Flow     |
| 36 | JZ          | 0x53   | Jump if Zero to Register pointer               | JZ  Reg      | Control Flow     |
| 37 | JNZ         | 0x54   | Jump if Not Zero to Literal memory address     | JNZ Lit      | Control Flow     |
| 38 | JNZ         | 0x55   | Jump if Not Zero to Register pointer           | JNZ Reg      | Control Flow     |
| 39 | JN          | 0x56   | Jump if Negative to Literal memory address     | JN  Lit      | Control Flow     |
| 40 | JN          | 0x57   | Jump if Negative to Register pointer           | JN  Reg      | Control Flow     |
| 41 | JNN         | 0x58   | Jump if Not Negative to Literal memory address | JNN Lit      | Control Flow     |
| 42 | JNN         | 0x59   | Jump if Not Negative to Register pointer       | JNN Reg      | Control Flow     |
| 43 | JC          | 0x5A   | Jump if Carry to Literal memory address        | JC  Lit      | Control Flow     |
| 44 | JC          | 0x5B   | Jump if Carry to Register pointer              | JC  Reg      | Control Flow     |
| 45 | JNC         | 0x5C   | Jump if Not Carry to Literal memory address    | JNC Lit      | Control Flow     |
| 46 | JNC         | 0x5D   | Jump if Not Carry to Register pointer          | JNC Reg      | Control Flow     |
| 47 | JSB         | 0x5E   | Jump to subroutine                             | JSB Lit      | Control Flow     |
| 48 | RSB         | 0x5F   | Return from subroutine                         | RSB          | Control Flow     |
| 49 | CLI         | 0x60   | Clear interrupt disabled flag                  | CLI          | Interrupt Handle |
| 50 | SEI         | 0x61   | Set interrupt disabled flag                    | SEI          | Interrupt Handle |
| 51 | RSI         | 0x62   | Return from interrupt subroutine               | RSI          | Interrupt Handle |

## Cupanasm (.casm)

Liguagem assembly que é usada para gerar código de cupana machine.

### Instrucions

| #  | INSTRUCTION | DESCRIPTION                                      | TYPE             | ARGS                 |
|----|-------------|--------------------------------------------------|------------------|----------------------|
| 1  | NOP         | No Operation                                     | Control          | None                 |
| 2  | HLT         | Halt                                             | Control          | None                 |
| 3  | MOV         | Move values betwwen registers, memory or literal | Data Transfer    | Reg, Reg* or literal |
| 4  | PHR         | Push register value to stack                     | Data Transfer    | Reg                  |
| 5  | PLR         | Pull value from stack                            | Data Transfer    | Reg                  |
| 6  | ADD         | Add two values                                   | Arithmetic       | Reg + Reg or literal |
| 7  | SUB         | Subtract two value                               | Arithmetic       | Reg + Reg or literal |
| 8  | MUL         | Multiply two value                               | Arithmetic       | Reg + Reg or literal |
| 9  | DIV         | Divide two value                                 | Arithmetic       | Reg + Reg or literal |
| 10 | MOD         | Modulo two value                                 | Arithmetic       | Reg + Reg or literal |
| 11 | INC         | Increment a Register value                       | Arithmetic       | Reg                  |
| 12 | DEC         | Decrement a Register value                       | Arithmetic       | Reg                  |
| 13 | AND         | Bitwise AND between two registers values         | Logic            | Reg + Reg or literal |
| 14 | OR          | Bitwise OR between two registers values          | Logic            | Reg + Reg or literal |
| 15 | XOR         | Bitwise XOR between two registers values         | Logic            | Reg + Reg or literal |
| 16 | NOT         | Bitwise NOT between two registers values         | Logic            | Reg                  |
| 17 | CMP         | Compare two values and update flags              | Comparison       | Reg or literal       |
| 18 | JMP         | Unconditional Jump to memory address             | Control Flow     | Reg or literal       |
| 19 | JZ          | Jump if Zero to  memory address                  | Control Flow     | Reg or literal       |
| 20 | JNZ         | Jump if Not Zero to memory address               | Control Flow     | Reg or literal       |
| 21 | JN          | Jump if Negative to memory address               | Control Flow     | Reg or literal       |
| 22 | JNN         | Jump if Not Negative to memory address           | Control Flow     | Reg or literal       |
| 23 | JC          | Jump if Carry to memory address                  | Control Flow     | Reg or literal       |
| 24 | JNC         | Jump if Not Carry to memory address              | Control Flow     | Reg or literal       |
| 25 | JSB         | Jump to subroutine                               | Control Flow     | Literal              |
| 26 | RSB         | Return from subroutine                           | Control Flow     | None                 |
| 27 | CLI         | Clear interrupt disabled flag                    | Interrupt Handle | None                 |
| 28 | SEI         | Set interrupt disabled flag                      | Interrupt Handle | None                 |
| 29 | RSI         | Return from interrupt subroutine                 | Interrupt Handle | None                 |

where:

| Arg       | Format               |
|-----------|----------------------|
| LITERAL   | $decimal or #hex |
| REGISTER  | Rid                |
| REGISTER* | Rid*               |

### Language

```casm
; coment

; Instructions and arguments are separated by space
mov r0 r1
add r1 $42

; alias is like a variable, you can use !(alias) to refer to register or literals. Can be exported
!alias Reg or Literal

; Label definition to set a point where the program counter comes when used in a call instruction.
; The assembler resolves the address direct to cupana machine code
label:

; Directives is used to tell the assembler to perform diferent actions on the cupana machine code
; .org tells the compiler where to put the code above with a rom memory addr
.org #addr
; .word define a label, u16 or a sequence of u16 null terminated value on current location
.word label
.word $decimal
.word #hex
.word "Hello World!"

```

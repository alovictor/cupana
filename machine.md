# Cupana System

- 16bits
- 64kb de memória
- Little Endian
- 14 registradores de uso geral: R0 - R13
- 2 registradores especiais:
  - R14: PC
  - R15: SP

## Address Space

| Type    | Range               | Size |
| ------- | ------------------- | ---- |
| ROM     | `0x0000` - `0x7FFF` | 32kb |
| RAM     | `0x8000` - `0xDFFF` | 28kb |
| STACK   | `0xE000` - `0xEFFF` | 4kb  |
| Devices | `0xF000` - `0xFFFF` | 4kb  |

## Stack

A stack cresce ascendente.

## Vector Table

| Endereço | Valor  | Descrição                |
| -------- | ------ | ------------------------ |
| 0x0000   | 0x0100 | Reset Vector             |
| 0x0002   | 0x0000 | Interrupt Routine Vector |

## Flags (16 bits)

| NAME               | Hex      |
| ------------------ | -------- |
| Zero               | `0x0001` |
| Negative           | `0x0002` |
| Overflow           | `0x0004` |
| Interrupt Disabled | `0x0008` |
| Interrupt Pending  | `0x0010` |
| Halt               | `0x0080` |

## Interrupções

Quando uma interrupção externa é aceita pelo processador, o hardware realiza automaticamente a preservação do contexto mínimo e desabilita novas interrupções.

## Instruções

| #   | OPCODE | HEX    | DESCRIPTION                 | TYPE             |
| --- | ------ | ------ | --------------------------- | ---------------- |
| 1   | NOP    | `0x00` | No Operation                | Control          |
| 2   | HLT    | `0x01` | Halt execution              | Control          |
| 3   | MOV    | `0x02` | Move data                   | Data Transfer    |
| 4   | PHR    | `0x03` | Push register to stack      | Data Transfer    |
| 5   | PLR    | `0x04` | Pull from stack to register | Data Transfer    |
| 6   | ADD    | `0x05` | Add                         | Arithmetic       |
| 7   | SUB    | `0x06` | Subtract                    | Arithmetic       |
| 8   | MUL    | `0x07` | Multiply                    | Arithmetic       |
| 9   | DIV    | `0x08` | Divide                      | Arithmetic       |
| 10  | MOD    | `0x09` | Modulo                      | Arithmetic       |
| 11  | INC    | `0x0A` | Increment                   | Arithmetic       |
| 12  | DEC    | `0x0B` | Decrement                   | Arithmetic       |
| 13  | AND    | `0x0C` | Bitwise AND                 | Logic            |
| 14  | OR     | `0x0D` | Bitwise OR                  | Logic            |
| 15  | XOR    | `0x0E` | Bitwise XOR                 | Logic            |
| 18  | NOT    | `0x0F` | Bitwise NOT                 | Logic            |
| 16  | SHL    | `0x10` | Shift Left                  | Logic            |
| 17  | SHR    | `0x11` | Shift Right                 | Logic            |
| 19  | CMP    | `0x12` | Compare                     | Comparison       |
| 20  | JMP    | `0x13` | Unconditional jump          | Control Flow     |
| 21  | JPC    | `0x14` | Jump on conditional         | Control Flow     |
| 22  | JSB    | `0x15` | Jump to Subroutine          | Control Flow     |
| 23  | RSB    | `0x16` | Return from Subroutine      | Control Flow     |
| 24  | CLI    | `0x17` | Clear Interrupt Enable      | Interrupt Handle |
| 25  | SEI    | `0x18` | Set Interrupt Enable        | Interrupt Handle |
| 26  | RSI    | `0x19` | Return from Interrupt       | Interrupt Handle |

As instruções Aritméticas e Lógicas sempre retornam o resultado no primeiro registrador argumento (Reg)

### Formato da Instrução

| Campo  | Bits   |
| ------ | ------ |
| OPCODE | 5 bits |
| B      | 1 bit  |
| M      | 2 bits |

- Caso a operação seja em bytes (B = 1), o byte menos significativo é preservado e o byte mais alto é zerado.

#### Modos de endereçamento

| Binário | Modo                         | Descrição  |
| ------- | ---------------------------- | ---------- |
| 00      | Registrador Direto           | REG, REG   |
| 01      | Registrador Imediato         | REG, LIT   |
| 10      | Registrador Indireto Destino | REG\*, REG |
| 11      | Registrador Indireto Origem  | REG, REG\* |

#### Modos de pulo condicional

| Hex    | Modo         |
| ------ | ------------ |
| `0x00` | Zero         |
| `0x01` | Not Zero     |
| `0x02` | Negative     |
| `0x03` | Not Negative |
| `0x04` | Overflow     |
| `0x05` | Not Overflow |

#### Instruções disponíveis e seus modos

| OPCODE | B   | Ma  | Mb  | Bin         | Hex  | ARG 1 | ARG 2 | Tamanho |
| :----- | :-- | :-- | :-- | :---------- | :--- | :---- | :---- | :------ |
| NOP    | 0   | 0   | 0   | 0b0000_0000 | 0x00 | -     | -     | 8 bits  |
| HLT    | 0   | 0   | 0   | 0b0000_1000 | 0x08 | -     | -     | 8 bits  |
| MOV    | 0   | 0   | 0   | 0b0001_0000 | 0x10 | Reg   | Reg   | 16 bits |
| MOV    | 0   | 0   | 1   | 0b0001_0001 | 0x11 | Reg   | Lit   | 32 bits |
| MOV    | 0   | 1   | 0   | 0b0001_0010 | 0x12 | Reg\* | Reg   | 16 bits |
| MOV    | 0   | 1   | 1   | 0b0001_0011 | 0x13 | Reg   | Reg\* | 16 bits |
| MOV    | 1   | 0   | 0   | 0b0001_0100 | 0x14 | Reg   | Reg   | 16 bits |
| MOV    | 1   | 0   | 1   | 0b0001_0101 | 0x15 | Reg   | Lit   | 24 bits |
| MOV    | 1   | 1   | 0   | 0b0001_0111 | 0x17 | Reg\* | Reg   | 16 bits |
| PHR    | 0   | 0   | 0   | 0b0001_1000 | 0x18 | Reg   | -     | 16 bits |
| PLR    | 0   | 0   | 0   | 0b0010_0000 | 0x20 | Reg   | -     | 16 bits |
| ADD    | 0   | 0   | 0   | 0b0010_1000 | 0x28 | Reg   | Reg   | 16 bits |
| ADD    | 0   | 0   | 1   | 0b0010_1001 | 0x29 | Reg   | Lit   | 32 bits |
| ADD    | 1   | 0   | 0   | 0b0010_1100 | 0x2C | Reg   | Reg   | 16 bits |
| ADD    | 1   | 0   | 1   | 0b0010_1101 | 0x2D | Reg   | Lit   | 24 bits |
| SUB    | 0   | 0   | 0   | 0b0011_0000 | 0x30 | Reg   | Reg   | 16 bits |
| SUB    | 0   | 0   | 1   | 0b0011_0001 | 0x31 | Reg   | Lit   | 32 bits |
| SUB    | 1   | 0   | 0   | 0b0011_0100 | 0x34 | Reg   | Reg   | 16 bits |
| SUB    | 1   | 0   | 1   | 0b0011_0101 | 0x35 | Reg   | Lit   | 24 bits |
| MUL    | 0   | 0   | 0   | 0b0011_1000 | 0x38 | Reg   | Reg   | 16 bits |
| MUL    | 0   | 0   | 1   | 0b0011_1001 | 0x39 | Reg   | Lit   | 32 bits |
| MUL    | 1   | 0   | 0   | 0b0011_1100 | 0x3C | Reg   | Reg   | 16 bits |
| MUL    | 1   | 0   | 1   | 0b0011_1101 | 0x3D | Reg   | Lit   | 24 bits |
| DIV    | 0   | 0   | 0   | 0b0100_0000 | 0x40 | Reg   | Reg   | 16 bits |
| DIV    | 0   | 0   | 1   | 0b0100_0001 | 0x41 | Reg   | Lit   | 32 bits |
| DIV    | 1   | 0   | 0   | 0b0100_0100 | 0x44 | Reg   | Reg   | 16 bits |
| DIV    | 1   | 0   | 1   | 0b0100_0101 | 0x45 | Reg   | Lit   | 24 bits |
| MOD    | 0   | 0   | 0   | 0b0100_1000 | 0x48 | Reg   | Reg   | 16 bits |
| MOD    | 0   | 0   | 1   | 0b0100_1001 | 0x49 | Reg   | Lit   | 32 bits |
| MOD    | 1   | 0   | 0   | 0b0100_1100 | 0x4C | Reg   | Reg   | 16 bits |
| MOD    | 1   | 0   | 1   | 0b0100_1101 | 0x4D | Reg   | Lit   | 24 bits |
| INC    | 0   | 0   | 0   | 0b0101_0000 | 0x50 | Reg   | -     | 16 bits |
| INC    | 1   | 0   | 0   | 0b0101_0100 | 0x54 | Reg   | -     | 16 bits |
| DEC    | 0   | 0   | 0   | 0b0101_1000 | 0x58 | Reg   | -     | 16 bits |
| DEC    | 1   | 0   | 0   | 0b0101_1100 | 0x5C | Reg   | -     | 16 bits |
| AND    | 0   | 0   | 0   | 0b0110_0000 | 0x60 | Reg   | Reg   | 16 bits |
| AND    | 0   | 0   | 1   | 0b0110_0001 | 0x61 | Reg   | Lit   | 32 bits |
| AND    | 1   | 0   | 0   | 0b0110_0100 | 0x64 | Reg   | Reg   | 16 bits |
| AND    | 1   | 0   | 1   | 0b0110_0101 | 0x65 | Reg   | Lit   | 24 bits |
| OR     | 0   | 0   | 0   | 0b0110_1000 | 0x68 | Reg   | Reg   | 16 bits |
| OR     | 0   | 0   | 1   | 0b0110_1001 | 0x69 | Reg   | Lit   | 32 bits |
| OR     | 1   | 0   | 0   | 0b0110_1100 | 0x6C | Reg   | Reg   | 16 bits |
| OR     | 1   | 0   | 1   | 0b0110_1101 | 0x6D | Reg   | Lit   | 24 bits |
| XOR    | 0   | 0   | 0   | 0b0111_0000 | 0x70 | Reg   | Reg   | 16 bits |
| XOR    | 0   | 0   | 1   | 0b0111_0001 | 0x71 | Reg   | Lit   | 32 bits |
| XOR    | 1   | 0   | 0   | 0b0111_0100 | 0x74 | Reg   | Reg   | 16 bits |
| XOR    | 1   | 0   | 1   | 0b0111_0101 | 0x75 | Reg   | Lit   | 24 bits |
| NOT    | 0   | 0   | 0   | 0b0111_1000 | 0x78 | Reg   | -     | 16 bits |
| NOT    | 1   | 0   | 0   | 0b0111_1100 | 0x7C | Reg   | -     | 16 bits |
| SHL    | 0   | 0   | 0   | 0b1000_0000 | 0x80 | Reg   | Reg   | 16 bits |
| SHL    | 0   | 0   | 1   | 0b1000_0001 | 0x81 | Reg   | Lit   | 32 bits |
| SHL    | 1   | 0   | 0   | 0b1000_0100 | 0x84 | Reg   | Reg   | 16 bits |
| SHL    | 1   | 0   | 1   | 0b1000_0101 | 0x85 | Reg   | Lit   | 24 bits |
| SHR    | 0   | 0   | 0   | 0b1000_1000 | 0x88 | Reg   | Reg   | 16 bits |
| SHR    | 0   | 0   | 1   | 0b1000_1001 | 0x89 | Reg   | Lit   | 32 bits |
| SHR    | 1   | 0   | 0   | 0b1000_1100 | 0x8C | Reg   | Reg   | 16 bits |
| SHR    | 1   | 0   | 1   | 0b1000_1101 | 0x8D | Reg   | Lit   | 24 bits |
| CMP    | 0   | 0   | 0   | 0b1001_0000 | 0x90 | Reg   | Reg   | 16 bits |
| CMP    | 0   | 0   | 1   | 0b1001_0001 | 0x91 | Reg   | Lit   | 32 bits |
| CMP    | 1   | 0   | 0   | 0b1001_0100 | 0x94 | Reg   | Reg   | 16 bits |
| CMP    | 1   | 0   | 1   | 0b1001_0101 | 0x95 | Reg   | Lit   | 24 bits |
| JMP    | 0   | 1   | 0   | 0b1001_1000 | 0x98 | Reg\* | -     | 16 bits |
| JMP    | 0   | 0   | 1   | 0b1001_1001 | 0x99 | Lit   | -     | 24 bits |
| JPC    | 0   | 0   | 0   | 0b1010_0000 | 0xA0 | Mode  | Reg   | 16 bits |
| JPC    | 0   | 0   | 1   | 0b1010_0001 | 0xA1 | Mode  | Lit   | 32 bits |
| JSB    | 0   | 1   | 0   | 0b1010_1000 | 0xA8 | Reg\* | -     | 24 bits |
| JSB    | 0   | 0   | 1   | 0b1010_1001 | 0xA9 | Lit   | -     | 24 bits |
| RSB    | 0   | 0   | 0   | 0b1011_1000 | 0xB8 | -     | -     | 8 bits  |
| CLI    | 0   | 0   | 0   | 0b1100_0000 | 0xC0 | -     | -     | 8 bits  |
| SEI    | 0   | 0   | 0   | 0b1100_1000 | 0xC8 | -     | -     | 8 bits  |
| RSI    | 0   | 0   | 0   | 0b1101_0000 | 0xD0 | -     | -     | 8 bits  |

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
| 24  | CLI    | `0x17` | Clear Interrupt flag        | Interrupt Handle |
| 25  | SEI    | `0x18` | Set Interrupt flag          | Interrupt Handle |
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
| `0x02` | Carry        |
| `0x03` | Not Carry    |
| `0x04` | Negative     |
| `0x05` | Not Negative |
| `0x06` | Overflow     |
| `0x07` | Not Overflow |

#### Instruções disponíveis e seus modos

| OPCODE | B   | Ma  | Mb  | Byte        | ARG 1 | ARG 2 | Tamanho |
| ------ | --- | --- | --- | ----------- | ----- | ----- | ------- |
| NOP    | 0   | 0   | 0   | 0b0000_0000 | -     | -     | 8 bits  |
| HLT    | 0   | 0   | 0   | 0b0000_1000 | -     | -     | 8 bits  |
| MOV    | 0   | 0   | 0   | 0b0001_0000 | Reg   | Reg   | 16 bits |
| MOV    | 0   | 0   | 1   | 0b0001_0001 | Reg   | Lit   | 32 bits |
| MOV    | 0   | 1   | 0   | 0b0001_0010 | Reg\* | Reg   | 16 bits |
| MOV    | 0   | 1   | 1   | 0b0001_0011 | Reg   | Reg\* | 16 bits |
| MOV    | 1   | 0   | 0   | 0b0001_0100 | Reg   | Reg   | 16 bits |
| MOV    | 1   | 0   | 1   | 0b0001_0101 | Reg   | Lit   | 24 bits |
| MOV    | 1   | 1   | 0   | 0b0001_0111 | Reg\* | Reg   | 16 bits |
| PHR    | 0   | 0   | 0   | 0b0001_1000 | Reg   | -     | 16 bits |
| PLR    | 0   | 0   | 0   | 0b0010_0000 | Reg   | -     | 16 bits |
| ADD    | 0   | 0   | 0   | 0b0010_1000 | Reg   | Reg   | 16 bits |
| ADD    | 0   | 0   | 1   | 0b0010_1001 | Reg   | Lit   | 32 bits |
| ADD    | 1   | 0   | 0   | 0b0010_1100 | Reg   | Reg   | 16 bits |
| ADD    | 1   | 0   | 1   | 0b0010_1101 | Reg   | Lit   | 24 bits |
| SUB    | 0   | 0   | 0   | 0b0011_0000 | Reg   | Reg   | 16 bits |
| SUB    | 0   | 0   | 1   | 0b0011_0001 | Reg   | Lit   | 32 bits |
| SUB    | 1   | 0   | 0   | 0b0011_0100 | Reg   | Reg   | 16 bits |
| SUB    | 1   | 0   | 1   | 0b0011_0101 | Reg   | Lit   | 24 bits |
| MUL    | 0   | 0   | 0   | 0b0011_1000 | Reg   | Reg   | 16 bits |
| MUL    | 0   | 0   | 1   | 0b0011_1001 | Reg   | Lit   | 32 bits |
| MUL    | 1   | 0   | 0   | 0b0011_1100 | Reg   | Reg   | 16 bits |
| MUL    | 1   | 0   | 1   | 0b0011_1101 | Reg   | Lit   | 24 bits |
| DIV    | 0   | 0   | 0   | 0b0100_0000 | Reg   | Reg   | 16 bits |
| DIV    | 0   | 0   | 1   | 0b0100_0001 | Reg   | Lit   | 32 bits |
| DIV    | 1   | 0   | 0   | 0b0100_0100 | Reg   | Reg   | 16 bits |
| DIV    | 1   | 0   | 1   | 0b0100_0101 | Reg   | Lit   | 24 bits |
| MOD    | 0   | 0   | 0   | 0b0100_1000 | Reg   | Reg   | 16 bits |
| MOD    | 0   | 0   | 1   | 0b0100_1001 | Reg   | Lit   | 32 bits |
| MOD    | 1   | 0   | 0   | 0b0100_1100 | Reg   | Reg   | 16 bits |
| MOD    | 1   | 0   | 1   | 0b0100_1101 | Reg   | Lit   | 24 bits |
| INC    | 0   | 0   | 0   | 0b0101_0000 | Reg   | -     | 16 bits |
| INC    | 1   | 0   | 0   | 0b0101_0100 | Reg   | -     | 16 bits |
| DEC    | 0   | 0   | 0   | 0b0101_1000 | Reg   | -     | 16 bits |
| DEC    | 1   | 0   | 0   | 0b0101_1100 | Reg   | -     | 16 bits |
| AND    | 0   | 0   | 0   | 0b0110_0000 | Reg   | Reg   | 16 bits |
| AND    | 0   | 0   | 1   | 0b0110_0001 | Reg   | Lit   | 32 bits |
| AND    | 1   | 0   | 0   | 0b0110_0100 | Reg   | Reg   | 16 bits |
| AND    | 1   | 0   | 1   | 0b0110_0101 | Reg   | Lit   | 24 bits |
| OR     | 0   | 0   | 0   | 0b0110_1000 | Reg   | Reg   | 16 bits |
| OR     | 0   | 0   | 1   | 0b0110_1001 | Reg   | Lit   | 32 bits |
| OR     | 1   | 0   | 0   | 0b0110_1100 | Reg   | Reg   | 16 bits |
| OR     | 1   | 0   | 1   | 0b0110_1101 | Reg   | Lit   | 24 bits |
| XOR    | 0   | 0   | 0   | 0b0111_0000 | Reg   | Reg   | 16 bits |
| XOR    | 0   | 0   | 1   | 0b0111_0001 | Reg   | Lit   | 32 bits |
| XOR    | 1   | 0   | 0   | 0b0111_0100 | Reg   | Reg   | 16 bits |
| XOR    | 1   | 0   | 1   | 0b0111_0101 | Reg   | Lit   | 24 bits |
| NOT    | 0   | 0   | 0   | 0b0111_1000 | Reg   | -     | 16 bits |
| NOT    | 1   | 0   | 0   | 0b0111_1100 | Reg   | -     | 16 bits |
| SHL    | 0   | 0   | 0   | 0b1000_0000 | Reg   | Reg   | 16 bits |
| SHL    | 0   | 0   | 1   | 0b1000_0001 | Reg   | Lit   | 32 bits |
| SHL    | 1   | 0   | 0   | 0b1000_0100 | Reg   | Reg   | 16 bits |
| SHL    | 1   | 0   | 1   | 0b1000_0101 | Reg   | Lit   | 24 bits |
| SHR    | 0   | 0   | 0   | 0b1000_1000 | Reg   | Reg   | 16 bits |
| SHR    | 0   | 0   | 1   | 0b1000_1001 | Reg   | Lit   | 32 bits |
| SHR    | 1   | 0   | 0   | 0b1000_1100 | Reg   | Reg   | 16 bits |
| SHR    | 1   | 0   | 1   | 0b1000_1101 | Reg   | Lit   | 24 bits |
| CMP    | 0   | 0   | 0   | 0b1001_0000 | Reg   | Reg   | 16 bits |
| CMP    | 0   | 0   | 1   | 0b1001_0001 | Reg   | Lit   | 32 bits |
| CMP    | 1   | 0   | 0   | 0b1001_0100 | Reg   | Reg   | 16 bits |
| CMP    | 1   | 0   | 1   | 0b1001_0101 | Reg   | Lit   | 24 bits |
| JMP    | 0   | 0   | 0   | 0b1001_1000 | Reg   | -     | 16 bits |
| JMP    | 0   | 0   | 1   | 0b1001_1001 | Lit   | -     | 24 bits |
| JPC    | 0   | 0   | 0   | 0b1010_0000 | Mode  | Reg   | 16 bits |
| JPC    | 0   | 0   | 1   | 0b1010_0001 | Mode  | Lit   | 32 bits |
| JSB    | 0   | 0   | 0   | 0b1010_1000 | Reg   | -     | 24 bits |
| JSB    | 0   | 0   | 1   | 0b1010_1001 | Lit   | -     | 24 bits |
| RSB    | 0   | 0   | 0   | 0b1011_1000 | -     | -     | 8 bits  |
| CLI    | 0   | 0   | 0   | 0b1100_0000 | -     | -     | 8 bits  |
| SEI    | 0   | 0   | 0   | 0b1100_1000 | -     | -     | 8 bits  |
| RSI    | 0   | 0   | 0   | 0b1101_0000 | -     | -     | 8 bits  |

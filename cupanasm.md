# Documentação da Linguagem Assembly Cupanasm (`.casm`)

## 1. Introdução

**Cupanasm** é a linguagem assembly para a **cupana machine**, uma arquitetura de processador virtual de 16-bit. Esta documentação descreve a arquitetura do processador, a sintaxe da linguagem, os modos de endereçamento e o conjunto completo de instruções (ISA - Instruction Set Architecture).

---

## 2. Arquitetura e Registradores

A `cupana machine` é um processador de 16-bit com uma arquitetura simples. Ela possui os seguintes registradores:

* **Registradores de Propósito Geral (16-bit):** `r0`, `r1`, `r2`, ... `r13`. São utilizados para manipulação geral de dados.
* **Registradores Especiais (16-bit):**
    * `pc` (Program Counter): Aponta para o endereço da próxima instrução a ser executada.
    * `sp` (Stack Pointer): Aponta para o topo da pilha (stack).
    * `flags`: Armazena o estado da CPU após operações, como:
        * **Z (Zero Flag):** Definida se o resultado de uma operação for zero.
        * **N (Negative Flag):** Definida se o resultado for negativo.
        * **C (Carry Flag):** Definida se uma operação gerou um "vai um" (carry).

---

## 3. Sintaxe e Modos de Endereçamento

A sintaxe geral de uma linha de código é: `rótulo: INSTRUÇÃO operando1, operando2 ; comentário`

### Modos de Endereçamento (Operandos)

| Notação | Modo | Descrição | Exemplo |
| :--- | :--- | :--- | :--- |
| `123` ou `0x7B` ou `0b0101` | **Imediato (Literal)** | O valor é um número fornecido diretamente na instrução. | `MOV r0, 42` |
| `rX` | **Registrador (Direto)** | O valor está contido em um registrador. | `MOV r1, r0` |
| `rX*` | **Registrador (Indireto)** | O registrador contém o **endereço de memória** do valor. | `MOV r1*, r0` |

---

## 4. Diretivas do Montador (Assembler)

Diretivas são comandos para o montador que não se traduzem diretamente em opcodes, mas controlam o processo de compilação.

* **`!alias`**: Define um nome (alias) para um registrador ou valor literal, funcionando como uma constante.
    ```casm
    !contador r0
    !inicio 0x1000
    MOV !contador, !inicio ; Equivale a: MOV r0, 0x1000
    ```

* **`.org`**: Define a "origem", ou seja, o endereço de memória inicial onde o código a seguir será colocado.
    ```casm
    .org 0x100 ; O código seguinte será montado a partir do endereço 256.
    ```

* **`.short`**: Aloca e inicializa uma ou mais palavras de 16-bit na memória. Pode ser usado para constantes ou strings.
    ```casm
    valor_config: .word 0xFF00
    mensagem:     .word "Ola!" ; Cria uma sequência de words, terminada em nulo.
    ```

* **`.byte`**: Aloca e inicializa um ou mais bytes de 8-bit na memória.
    ```casm
    idade: .byte 30
    ```

---

## 5. Conjunto de Instruções (ISA)

### Controle (Control)

| # | Opcode (Hex) | Mnemônico | Descrição |
|---|---|---|---|
| 1 | 0x01 | `NOP` | Nenhuma Operação. |
| 2 | 0x02 | `HLT` | Para a execução do processador. |

### Transferência de Dados (Data Transfer)

| # | Opcode (Hex) | Mnemônico | Descrição | Operandos (destino, origem) |
|---|---|---|---|---|
| 3 | 0x03 | `MOV` | Move um valor de 16-bit (word). | `reg_dest, reg_orig` / `reg_dest, literal` / `reg_dest_ptr*, reg_orig` |
| 4 | 0x04 | `MOVB` | Move um valor de 8-bit (byte). | `reg_dest, reg_orig` / `reg_dest, literal` / `reg_dest_ptr*, reg_orig` |
| 5 | 0x05 | `PHR` | Empurra o valor de um registrador para a pilha (push). | `reg` |
| 6 | 0x06 | `PLR` | Puxa um valor da pilha para um registrador (pull). | `reg` |

### Aritmética (Arithmetic)

| # | Opcode (Hex) | Mnemônico | Descrição | Operandos (destino, origem) |
|---|---|---|---|---|
| 7 | 0x07 | `ADD` | Soma dois valores de 16-bit. O resultado fica no destino. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 8 | 0x08 | `ADDB` | Soma dois valores de 8-bit. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 9 | 0x09 | `SUB` | Subtrai dois valores de 16-bit. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 10 | 0x0A | `SUBB` | Subtrai dois valores de 8-bit. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 11 | 0x0B | `MUL` | Multiplica dois valores de 16-bit. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 12 | 0x0C | `MULB` | Multiplica dois valores de 8-bit. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 13 | 0x0D | `DIV` | Divide dois valores de 16-bit. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 14 | 0x0E | `DIVB` | Divide dois valores de 8-bit. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 15 | 0x0F | `MOD` | Calcula o módulo de dois valores de 16-bit. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 16 | 0x10 | `MODB` | Calcula o módulo de dois valores de 8-bit. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 17 | 0x11 | `INC` | Incrementa o valor de um registrador de 16-bit. | `reg` |
| 18 | 0x12 | `INCB` | Incrementa o valor de um registrador de 8-bit. | `reg` |
| 19 | 0x13 | `DEC` | Decrementa o valor de um registrador de 16-bit. | `reg` |
| 20 | 0x14 | `DECB` | Decrementa o valor de um registrador de 8-bit. | `reg` |

### Lógica (Logic)

| # | Opcode (Hex) | Mnemônico | Descrição | Operandos (destino, origem) |
|---|---|---|---|---|
| 21 | 0x15 | `AND` | Operação "E" bit a bit (Bitwise AND) em 16-bit. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 22 | 0x16 | `ANDB` | Operação "E" bit a bit (Bitwise AND) em 8-bit. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 23 | 0x17 | `OR` | Operação "OU" bit a bit (Bitwise OR) em 16-bit. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 24 | 0x18 | `ORB` | Operação "OU" bit a bit (Bitwise OR) em 8-bit. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 25 | 0x19 | `SHL` | Desloca os bits para a esquerda (Shift Left). | `reg_dest, reg_orig` / `reg_dest, literal` |
| 26 | 0x1A | `SHR` | Desloca os bits para a direita (Shift Right). | `reg_dest, reg_orig` / `reg_dest, literal` |
| 27 | 0x1B | `NOT` | Negação bit a bit (Bitwise NOT) em 16-bit. | `reg` |
| 28 | 0x1C | `NOTB` | Negação bit a bit (Bitwise NOT) em 8-bit. | `reg` |

### Comparação (Comparison)

| # | Opcode (Hex) | Mnemônico | Descrição | Operandos |
|---|---|---|---|---|
| 29 | 0x1D | `CMP` | Compara dois valores de 16-bit e atualiza as `flags`. | `reg, reg` / `reg, literal` |
| 30 | 0x1E | `CMPB` | Compara dois valores de 8-bit e atualiza as `flags`. | `reg, reg` / `reg, literal` |

### Fluxo de Controle (Control Flow)

| # | Opcode (Hex) | Mnemônico | Descrição | Operandos |
|---|---|---|---|---|
| 31 | 0x1F | `JMP` | Salto incondicional para um endereço. | `endereço` |
| 32 | 0x20 | `JZ` | Salta se a flag Zero (Z) estiver ativa (Jump if Zero). | `endereço` |
| 33 | 0x21 | `JNZ` | Salta se a flag Zero (Z) não estiver ativa (Jump if Not Zero). | `endereço` |
| 34 | 0x22 | `JN` | Salta se a flag Negativo (N) estiver ativa (Jump if Negative). | `endereço` |
| 35 | 0x23 | `JNN` | Salta se a flag Negativo (N) não estiver ativa. | `endereço` |
| 36 | 0x24 | `JC` | Salta se a flag Carry (C) estiver ativa (Jump if Carry). | `endereço` |
| 37 | 0x25 | `JNC` | Salta se a flag Carry (C) não estiver ativa. | `endereço` |
| 38 | 0x26 | `JSB` | Salta para uma sub-rotina (guarda o endereço de retorno na pilha). | `endereço` |
| 39 | 0x27 | `RSB` | Retorna de uma sub-rotina (recupera o endereço da pilha). | - |

### Manipulação de Interrupções (Interrupt Handle)

| # | Opcode (Hex) | Mnemônico | Descrição |
|---|---|---|---|
| 40 | 0x28 | `CLI` | Limpa a flag de desabilitar interrupções (habilita interrupções). |
| 41 | 0x29 | `SEI` | Define a flag de desabilitar interrupções (ignora interrupções). |
| 42 | 0x2A | `RSI` | Retorna de uma sub-rotina de interrupção. |

---

## 6. Exemplo de Programa: Soma de Dois Números

Este programa soma dois números (`15` e `27`) definidos na memória e armazena o resultado em outra posição de memória.

```casm
; Inicia o código no endereço de memória 0x100
.org 0x100

inicio:
    ; Carrega o endereço da primeira variável em r1
    MOV r1, numero_a
    ; Carrega o valor apontado por r1 (15) para r0
    MOV r0, r1*

    ; Carrega o endereço da segunda variável em r1
    MOV r1, numero_b
    ; Adiciona o valor apontado por r1 (27) ao valor em r0
    ADD r0, r1*

    ; Carrega o endereço da variável de resultado em r1
    MOV r1, resultado
    ; Armazena o valor de r0 (42) no endereço de memória apontado por r1
    MOV r1*, r0

    ; Termina a execução do programa
    HLT

; =================================
; Seção de Dados
; =================================
.org 0x200 ; Coloca os dados em outra área da memória

numero_a:   .word 15
numero_b:   .word 27
resultado:  .word 0 ; Espaço para armazenar o resultado
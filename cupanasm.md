# Documentação da Linguagem Assembly Cupanasm (`.casm`)

## 1. Introdução

**Cupanasm** é a linguagem assembly para a **cupana machine**, uma arquitetura de processador virtual de 16-bit. Esta documentação descreve a arquitetura do processador, a sintaxe da linguagem, os modos de endereçamento e o conjunto completo de instruções em cupanasm.

---

## 2. Arquitetura e Registradores

A `cupana machine` é um processador de 16-bit LE com uma arquitetura simples. Ela possui os seguintes registradores:

* **Registradores de Propósito Geral (16-bit):** `R0`, `R1`, `R2`, ... `R13`. São utilizados para manipulação geral de dados.
* **Registradores Especiais (16-bit):**
  * `R14` ou `PC` (Program Counter): Aponta para o endereço da próxima instrução a ser executada.
  * `R15` ou `SP` (Stack Pointer): Aponta para o topo da pilha (stack).
  * `FLAGS` (16-bit): Armazena o estado da CPU após operações:
    * **Zero (0x0001)**: Definida se o resultado de uma operação for zero.
    * **Carry (0x0002)**: Definida se uma operação gerou um "vai um" (carry).
    * **Negative (0x0004)**: Definida se o resultado for negativo.
    * **Overflow (0x0008)**: Definida se uma operação resultou em um overflow.
    * **Interrupt Disabled (0x0010)**: Interrupções estão desabilitadas.
    * **Interrupt Pending (0x0020)**: Há uma interrupção pendente.
    * **Halt (0x0080):** Processador parou.

### Mapa de memória

| Type    | Range               | Size |
| ------- | ------------------- | ---- |
| ROM     | `0x0000` - `0x7FFF` | 32kb |
| RAM     | `0x8000` - `0xDFFF` | 28kb |
| STACK   | `0xE000` - `0xEFFF` | 4kb  |
| Devices | `0xF000` - `0xFFFF` | 4kb  |

---

## 3. Sintaxe e Modos de Endereçamento

A sintaxe geral de uma linha de código é: `rótulo: INSTRUÇÃO operando1, operando2 ; comentário`

### Modos de Endereçamento (Operandos)

| Notação                     | Modo                     | Descrição                                               | Exemplo                        |
| :-------------------------- | :----------------------- | :------------------------------------------------------ | :----------------------------- |
| `123` ou `0x7B` ou `0b0101` | **Imediato (Literal)**   | O valor é um número fornecido diretamente na instrução. | `MOV R0, 42`                   |
| `RX`                        | **Registrador Direto**   | O valor está contido em um registrador.                 | `MOV R1, R0`                   |
| `RX*`                       | **Registrador Indireto** | O registrador contém o endereço onde armazenar o valor. | `MOV R1*, R0` ou `MOV R1, R0*` |

---

## 4. Labels

Labels são referências a endereços na memória rom, que permite pular para pontos específicos do programa como funcões ou outros pontos arbitrários.

``` casm
label:
```

## 4. Diretivas do Montador (Assembler)

Diretivas são comandos para o montador que não se traduzem diretamente em opcodes, mas controlam o processo de compilação.

* **`.org`**: Define a "origem", ou seja, o endereço de memória inicial onde o código a seguir será colocado.
  
```casm
.org 0x100 ; O código seguinte será montado a partir do endereço 256.
```

* **`.include`**: Importa um arquivo casm.
  
```casm
.include "nome_do_arquivo.casm"
```

* **`.const`**: Define uma constante na memória.

```casm
nome: .const 30
```

* **`.short`**: Aloca e inicializa uma ou mais palavras de 16-bit na memória. Pode ser usado para constantes ou strings.

```casm
valor_config: .short 0xFF00
mensagem:     .short "Ola!" ; Cria uma sequência de words, terminada em nulo.
```

* **`.byte`**: Aloca e inicializa um ou mais bytes de 8-bit na memória.

```casm
idade: .byte 30
```

* **`.ascii`**: Aloca e inicializa sequência de caracteres em ascii

``` casm
nome: .ascii "João"
```

---

## 5. Instruções

### Controle (Control)

| #   | Instrução | Descrição                       |
| --- | --------- | ------------------------------- |
| 1   | `NOP`     | Nenhuma Operação.               |
| 2   | `HLT`     | Para a execução do processador. |

### Transferência de Dados (Data Transfer)

| #   | Instrução | Descrição                                              | Operandos (destino, origem)                                                                        |
| --- | --------- | ------------------------------------------------------ | -------------------------------------------------------------------------------------------------- |
| 3   | `MOV`     | Move um valor de 16-bit (word).                        | `reg_dest, reg_orig` / `reg_dest, literal` / `reg_dest_ptr*, reg_orig` / `reg_dest, reg_orig_ptr*` |
| 4   | `MOVB`    | Move um valor de 8-bit (byte).                         | `reg_dest, reg_orig` / `reg_dest, literal` / `reg_dest_ptr*, reg_orig` / `reg_dest, reg_orig_ptr*` |
| 5   | `PHR`     | Empurra o valor de um registrador para a pilha (push). | `reg`                                                                                              |
| 6   | `PLR`     | Puxa um valor da pilha para um registrador (pull).     | `reg`                                                                                              |

### Aritmética (Arithmetic)

| #   | Instrução | Descrição                                                 | Operandos (destino, origem)                |
| --- | --------- | --------------------------------------------------------- | ------------------------------------------ |
| 7   | `ADD`     | Soma dois valores de 16-bit. O resultado fica no destino. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 8   | `ADDB`    | Soma dois valores de 8-bit.                               | `reg_dest, reg_orig` / `reg_dest, literal` |
| 9   | `SUB`     | Subtrai dois valores de 16-bit.                           | `reg_dest, reg_orig` / `reg_dest, literal` |
| 10  | `SUBB`    | Subtrai dois valores de 8-bit.                            | `reg_dest, reg_orig` / `reg_dest, literal` |
| 11  | `MUL`     | Multiplica dois valores de 16-bit.                        | `reg_dest, reg_orig` / `reg_dest, literal` |
| 12  | `MULB`    | Multiplica dois valores de 8-bit.                         | `reg_dest, reg_orig` / `reg_dest, literal` |
| 13  | `DIV`     | Divide dois valores de 16-bit.                            | `reg_dest, reg_orig` / `reg_dest, literal` |
| 14  | `DIVB`    | Divide dois valores de 8-bit.                             | `reg_dest, reg_orig` / `reg_dest, literal` |
| 15  | `MOD`     | Calcula o módulo de dois valores de 16-bit.               | `reg_dest, reg_orig` / `reg_dest, literal` |
| 16  | `MODB`    | Calcula o módulo de dois valores de 8-bit.                | `reg_dest, reg_orig` / `reg_dest, literal` |
| 17  | `INC`     | Incrementa o valor de um registrador de 16-bit.           | `reg`                                      |
| 18  | `INCB`    | Incrementa o valor de um registrador de 8-bit.            | `reg`                                      |
| 19  | `DEC`     | Decrementa o valor de um registrador de 16-bit.           | `reg`                                      |
| 20  | `DECB`    | Decrementa o valor de um registrador de 8-bit.            | `reg`                                      |

### Lógica (Logic)

| #   | Instrução | Descrição                                        | Operandos (destino, origem)                |
| --- | --------- | ------------------------------------------------ | ------------------------------------------ |
| 21  | `AND`     | Operação "E" bit a bit (Bitwise AND) em 16-bit.  | `reg_dest, reg_orig` / `reg_dest, literal` |
| 22  | `ANDB`    | Operação "E" bit a bit (Bitwise AND) em 8-bit.   | `reg_dest, reg_orig` / `reg_dest, literal` |
| 23  | `OR`      | Operação "OU" bit a bit (Bitwise OR) em 16-bit.  | `reg_dest, reg_orig` / `reg_dest, literal` |
| 24  | `ORB`     | Operação "OU" bit a bit (Bitwise OR) em 8-bit.   | `reg_dest, reg_orig` / `reg_dest, literal` |
| 25  | `XOR`     | Operação "XOR" bit a bit (Bitwise OR) em 16-bit. | `reg_dest, reg_orig` / `reg_dest, literal` |
| 26  | `XORB`    | Operação "XOR" bit a bit (Bitwise OR) em 8-bit.  | `reg_dest, reg_orig` / `reg_dest, literal` |
| 27  | `SHL`     | Desloca os bits para a esquerda (Shift Left).    | `reg_dest, reg_orig` / `reg_dest, literal` |
| 28  | `SHLB`    | Desloca os bits para a esquerda (Shift Left).    | `reg_dest, reg_orig` / `reg_dest, literal` |
| 29  | `SHR`     | Desloca os bits para a direita (Shift Right).    | `reg_dest, reg_orig` / `reg_dest, literal` |
| 30  | `SHRB`     | Desloca os bits para a direita (Shift Right).    | `reg_dest, reg_orig` / `reg_dest, literal` |
| 31  | `NOT`     | Negação bit a bit (Bitwise NOT) em 16-bit.       | `reg`                                      |
| 32  | `NOTB`    | Negação bit a bit (Bitwise NOT) em 8-bit.        | `reg`                                      |

### Comparação (Comparison)

| #   | Instrução | Descrição                                             | Operandos                   |
| --- | --------- | ----------------------------------------------------- | --------------------------- |
| 33  | `CMP`     | Compara dois valores de 16-bit e atualiza as `flags`. | `reg, reg` / `reg, literal` |
| 34  | `CMPB`    | Compara dois valores de 8-bit e atualiza as `flags`.  | `reg, reg` / `reg, literal` |

### Fluxo de Controle (Control Flow)

| #   | Instrução | Descrição                                                          | Operandos  |
| --- | --------- | ------------------------------------------------------------------ | ---------- |
| 35  | `JMP`     | Salto incondicional para um endereço.                              | `endereço` |
| 36  | `JZ`      | Salta se a flag Zero (Z) estiver ativa (Jump if Zero).             | `endereço` |
| 37  | `JNZ`     | Salta se a flag Zero (Z) não estiver ativa (Jump if Not Zero).     | `endereço` |
| 38  | `JN`      | Salta se a flag Negativo (N) estiver ativa (Jump if Negative).     | `endereço` |
| 39  | `JNN`     | Salta se a flag Negativo (N) não estiver ativa.                    | `endereço` |
| 40  | `JC`      | Salta se a flag Carry (C) estiver ativa (Jump if Carry).           | `endereço` |
| 41  | `JNC`     | Salta se a flag Carry (C) não estiver ativa.                       | `endereço` |
| 42  | `JO`      | Salta se a flag Overflow (O) estiver ativa (Jump if Overflow).     | `endereço` |
| 43  | `JNO`     | Salta se a flag Overflow (O) não estiver ativa.                    | `endereço` |
| 44  | `JSB`     | Salta para uma sub-rotina (guarda o endereço de retorno na pilha). | `endereço` |
| 45  | `RSB`     | Retorna de uma sub-rotina (recupera o endereço da pilha).          | -          |

### Manipulação de Interrupções (Interrupt Handle)

| #   | Instrução | Descrição                                                         |
| --- | --------- | ----------------------------------------------------------------- |
| 46  | `CLI`     | Limpa a flag de desabilitar interrupções (habilita interrupções). |
| 47  | `SEI`     | Define a flag de desabilitar interrupções (ignora interrupções).  |
| 48  | `RSI`     | Retorna de uma sub-rotina de interrupção.                         |

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

numero_a:   .short 15
numero_b:   .short 27
resultado:  .short 0 ; Espaço para armazenar o resultado

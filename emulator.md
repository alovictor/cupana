# Cupana System

O sistema é feito por:

| Item            | Descrição               |
|-----------------|-------------------------|
| Cupana Machine  | CPU                     |
| Memory          | Memória                 |
| Screen Device   | Dispositivo de tela     |
| Keyboard Device | Dispositivo de teclado  |
| Mouse Device    | Dispositivo de mouse    |
| Sound Device    | Dispositivo de som      |
| File            | Dispositivo de arquivo  |
| Datetime        | Dispositivo de datetime |

## Cupana Machine

Uma cpu de 16 bits capaz de realizar diversas operações, como :

| Tipo                    | Operações                                |
|-------------------------|------------------------------------------|
| Movimentação de memória | mov, phr, plr                            |
| Aritmética              | add, sub, mul, div, mod                  |
| Lógica                  | and, or, shl, shr, not                   |
| Comparação              | cmp                                      |
| Controle de fluxo       | jmp, jz, jnz, jn, jnn, jc, jnc, jsb, rsb |
| Interrupções            | cli, sei, rsi                            |

O resultado das comparações é armazenado em flags internas ao processador:

| NAME     | POSITION      |
|----------|---------------|
| Zero     | `0b0000_0001` |
| Carry    | `0b0000_0010` |
| Negative | `0b0000_0100` |

## Memory

Memória de 64kb com o seguinte mapeamento:

| Type    | Range               | Size |
|---------|---------------------|------|
| ROM     | `0x0000` - `0x7FFF` | 32kb |
| RAM     | `0x8000` - `0xEFFF` | 28kb |
| Devices | `0xF000` - `0xFFFF` | 4kb  |

## Screen Device

## Keyboard Device

## Mouse Device

## Sound Device

## File

## Datetime

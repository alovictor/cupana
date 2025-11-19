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

Uma cpu de 16 bits capaz de realizar diversas operações.

## Memory

Memória de 64kb com o seguinte mapeamento:

| Type    | Range               | Size |
| ------- | ------------------- | ---- |
| ROM     | `0x0000` - `0x7FFF` | 32kb |
| RAM     | `0x8000` - `0xDFFF` | 28kb |
| STACK   | `0xE000` - `0xEFFF` | 4kb  |
| Devices | `0xF000` - `0xFFFF` | 4kb  |

## Console Device

## Keyboard Device

## File

## Datetime

## Screen Device

## Mouse Device

## Sound Device

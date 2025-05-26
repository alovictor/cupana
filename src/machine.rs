use std::fmt::Display;

use crate::{
    error::VMError,
    memory::{Memory, Ram, Rom},
};

const NUM_REGISTERS: usize = 0x10; // 16 registradores
const FLAG_ZERO: u8 = 0b0000_0001;
const FLAG_CARRY: u8 = 0b0000_0010;
const FLAG_NEGATIVE: u8 = 0b0000_0100;
const FLAG_HALT: u8 = 0b1000_0000;

pub struct CupanaMachine {
    registers: [u16; NUM_REGISTERS],
    stack: Vec<u16>,
    pc: u16,
    flags: u8,
}

impl CupanaMachine {
    pub fn new() -> Self {
        Self {
            registers: [0; NUM_REGISTERS],
            stack: Vec::new(),
            pc: 0x100,
            flags: 0,
        }
    }

    pub fn has_halted(&self) -> bool {
        self.flags & FLAG_HALT != 0
    }

    fn is_flag_set(&self, flag_mask: u8) -> bool {
        (self.flags & flag_mask) != 0
    }

    fn update_flag(&mut self, flag_mask: u8, value: bool) {
        if value {
            self.flags |= flag_mask;
        } else {
            self.flags &= !flag_mask;
        }
    }

    fn update_flags(&mut self, result: u16, carry_occurred: Option<bool>) {
        self.update_flag(FLAG_ZERO, result == 0);
        self.update_flag(FLAG_NEGATIVE, (result & 0x8000) != 0);
        if let Some(carry) = carry_occurred {
            self.update_flag(FLAG_CARRY, carry);
        } else {
            self.update_flag(FLAG_CARRY, false);
        }
    }

    fn get_register_index(&self, pc: u16, rom: &Rom) -> Result<usize, VMError> {
        let reg = rom.read_u8(pc)?;
        if reg < NUM_REGISTERS as u8 {
            Ok(reg as usize)
        } else {
            Err(VMError::InvalidRegister(reg))
        }
    }

    pub fn step(&mut self, rom: &Rom, ram: &mut Ram) -> Result<(), VMError> {
        let opcode = rom.read_u8(self.pc)?;
        match opcode {
            // NOP (0x00)
            0x00 => {
                self.pc += 1;
            }
            // HLT (0x01)
            0x01 => {
                self.update_flag(FLAG_HALT, true);
            }
            // MOV reg reg (0x10)
            0x10 => {
                let dest = self.get_register_index(self.pc + 1, rom)?;
                let source = self.get_register_index(self.pc + 2, rom)?;
                self.registers[dest] = self.registers[source];
                self.pc += 3;
            }
            // MOV reg lit (0x11)
            0x11 => {
                let dest_idx = self.get_register_index(self.pc + 1, rom)?;
                let source = rom.read_u16(self.pc + 2)?;
                self.registers[dest_idx] = source;
                self.pc += 4;
            }
            // MOV reg mem (0x12)
            0x12 => {
                let dest_idx = self.get_register_index(self.pc + 1, rom)?;
                let source = rom.read_u16(self.pc + 2)?;
                self.registers[dest_idx] = rom.read_u16(source)?;
                self.pc += 4;
            }
            // MOV reg reg* (0x13)
            0x13 => {
                let dest_idx = self.get_register_index(self.pc + 1, rom)?;
                let source_idx = self.get_register_index(self.pc + 2, rom)?;
                self.registers[dest_idx] = rom.read_u16(self.registers[source_idx])?;
                self.pc += 3;
            }
            // MOV mem reg (0x14)
            0x14 => {
                let dest = rom.read_u16(self.pc + 1)?;
                let source_idx = self.get_register_index(self.pc + 3, rom)?;
                ram.write_u16(dest, self.registers[source_idx])?;
                self.pc += 4;
            }
            // MOV reg* reg (0x15)
            0x15 => {
                let dest_idx = self.get_register_index(self.pc + 1, rom)?;
                let source_idx = self.get_register_index(self.pc + 2, rom)?;
                ram
                    .write_u16(self.registers[dest_idx], self.registers[source_idx])?;
                self.pc += 3;
            }
            // ADD reg reg (0x20)
            0x20 => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let source2_idx = self.get_register_index(self.pc + 2, rom)?;

                let val1 = self.registers[source1_idx];
                let val2 = self.registers[source2_idx];

                let (result, carry_occurred) = val1.overflowing_add(val2);
                self.registers[0] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 3;
            }
            // ADD reg lit (0x21)
            0x21 => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let val1 = self.registers[source1_idx];
                let val2 = rom.read_u16(self.pc + 2)?;

                let (result, carry_occurred) = val1.overflowing_add(val2);
                self.registers[0] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 4;
            }
            // SUB reg reg (0x22)
            0x22 => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let source2_idx = self.get_register_index(self.pc + 2, rom)?;

                let val1 = self.registers[source1_idx];
                let val2 = self.registers[source2_idx];

                let (result, carry_occurred) = val1.overflowing_sub(val2);
                self.registers[0] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 3;
            }
            // SUB reg lit (0x23)
            0x23 => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let val1 = self.registers[source1_idx];
                let val2 = rom.read_u16(self.pc + 2)?;

                let (result, carry_occurred) = val1.overflowing_sub(val2);
                self.registers[0] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 4;
            }
            // SUB lit reg (0x24)
            0x24 => {
                let val1 = rom.read_u16(self.pc + 1)?;
                let source2_idx = self.get_register_index(self.pc + 3, rom)?;
                let val2 = self.registers[source2_idx];

                let (result, carry_occurred) = val1.overflowing_sub(val2);
                self.registers[0] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 4;
            }
            // MUL reg reg (0x25)
            0x25 => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let source2_idx = self.get_register_index(self.pc + 2, rom)?;

                let val1 = self.registers[source1_idx];
                let val2 = self.registers[source2_idx];

                let (result, carry_occurred) = val1.overflowing_mul(val2);
                self.registers[0] = result;
                self.update_flags(result, Some(carry_occurred));

                self.pc += 3;
            }
            // MUL reg lit (0x26)
            0x26 => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let val1 = self.registers[source1_idx];
                let val2 = rom.read_u16(self.pc + 2)?;

                let (result, carry_occurred) = val1.overflowing_mul(val2);
                self.registers[0] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 4;
            }
            // DIV reg reg (0x27)
            0x27 => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let source2_idx = self.get_register_index(self.pc + 2, rom)?;

                let val1 = self.registers[source1_idx];
                let val2 = self.registers[source2_idx];

                if val2 == 0 {
                    return Err(VMError::DivideByZero); // Ou lidar via exceção se implementado
                } else {
                    let (result, carry_occurred) = val1.overflowing_div(val2);
                    self.registers[0] = result;

                    self.update_flags(result, Some(carry_occurred));
                    self.pc += 3;
                }
            }
            // DIV reg lit (0x28)
            0x28 => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let val1 = self.registers[source1_idx];
                let val2 = rom.read_u16(self.pc + 2)?;

                if val2 == 0 {
                    return Err(VMError::DivideByZero); // Ou lidar via exceção se implementado
                } else {
                    let (result, carry_occurred) = val1.overflowing_div(val2);
                    self.registers[0] = result;

                    self.update_flags(result, Some(carry_occurred));
                    self.pc += 4;
                }
            }
            // DIV lit reg (0x29)
            0x29 => {
                let val1 = rom.read_u16(self.pc + 1)?;
                let source2_idx = self.get_register_index(self.pc + 3, rom)?;
                let val2 = self.registers[source2_idx];

                if val2 == 0 {
                    return Err(VMError::DivideByZero); // Ou lidar via exceção se implementado
                } else {
                    let (result, carry_occurred) = val1.overflowing_div(val2);
                    self.registers[0] = result;

                    self.update_flags(result, Some(carry_occurred));
                    self.pc += 4;
                }
            }
            // MOD reg reg (0x2A)
            0x2A => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let source2_idx = self.get_register_index(self.pc + 2, rom)?;

                let val1 = self.registers[source1_idx];
                let val2 = self.registers[source2_idx];

                if val2 == 0 {
                    return Err(VMError::DivideByZero); // Ou lidar via exceção se implementado
                } else {
                    let (result, carry_occurred) = val1.overflowing_rem(val2);
                    self.registers[0] = result;

                    self.update_flags(result, Some(carry_occurred));
                    self.pc += 3;
                }
            }
            // MOD reg lit (0x2B)
            0x2B => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let val1 = self.registers[source1_idx];
                let val2 = rom.read_u16(self.pc + 2)?;

                if val2 == 0 {
                    return Err(VMError::DivideByZero); // Ou lidar via exceção se implementado
                } else {
                    let (result, carry_occurred) = val1.overflowing_rem(val2);
                    self.registers[0] = result;

                    self.update_flag(FLAG_ZERO, result == 0);
                    self.update_flag(FLAG_NEGATIVE, (result & 0x8000) != 0);
                    self.update_flag(FLAG_CARRY, carry_occurred);
                    self.pc += 4;
                }
            }
            // MOD lit reg (0x2C)
            0x2C => {
                let val1 = rom.read_u16(self.pc + 1)?;
                let source2_idx = self.get_register_index(self.pc + 3, rom)?;
                let val2 = self.registers[source2_idx];

                if val2 == 0 {
                    return Err(VMError::DivideByZero); // Ou lidar via exceção se implementado
                } else {
                    let (result, carry_occurred) = val1.overflowing_rem(val2);
                    self.registers[0] = result;

                    self.update_flags(result, Some(carry_occurred));
                    self.pc += 4;
                }
            }
            // INC (0x2D)
            0x2D => {
                let idx = self.get_register_index(self.pc + 1, rom)?;
                let val1 = self.registers[idx];

                let (result, carry_occurred) = val1.overflowing_add(1);
                self.registers[idx] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 2;
            }
            // DEC (0x2E)
            0x2E => {
                let idx = self.get_register_index(self.pc + 1, rom)?;
                let val1 = self.registers[idx];

                let (result, carry_occurred) = val1.overflowing_sub(1);
                self.registers[idx] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 2;
            }

            // AND reg reg (0x30)
            0x30 => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let source2_idx = self.get_register_index(self.pc + 2, rom)?;

                let val1 = self.registers[source1_idx];
                let val2 = self.registers[source2_idx];

                let result = val1 & val2;
                self.registers[0] = result;

                self.update_flags(result, None);
                self.pc += 3;
            }
            // OR reg reg (0x31)
            0x31 => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let source2_idx = self.get_register_index(self.pc + 2, rom)?;

                let val1 = self.registers[source1_idx];
                let val2 = self.registers[source2_idx];

                let result = val1 | val2;
                self.registers[0] = result;

                self.update_flags(result, None);
                self.pc += 3;
            }
            // XOR reg reg (0x32)
            0x32 => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let source2_idx = self.get_register_index(self.pc + 2, rom)?;

                let val1 = self.registers[source1_idx];
                let val2 = self.registers[source2_idx];

                let result = val1 ^ val2;
                self.registers[0] = result;

                self.update_flags(result, None);
                self.pc += 3;
            }
            // NOT reg (0x33)
            0x33 => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;

                let val1 = self.registers[source1_idx];

                let result = !val1;
                self.registers[0] = result;

                self.update_flags(result, None);
                self.pc += 2;
            }

            // CMP reg reg (0x40)
            0x40 => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let source2_idx = self.get_register_index(self.pc + 2, rom)?;

                let val1 = self.registers[source1_idx];
                let val2 = self.registers[source2_idx];

                let (result, carry_occurred) = val1.overflowing_sub(val2);

                self.update_flags(result, Some(carry_occurred));
                self.pc += 3;
            }
            // CMP reg lit (0x41)
            0x41 => {
                let source1_idx = self.get_register_index(self.pc + 1, rom)?;
                let val1 = self.registers[source1_idx];
                let val2 = rom.read_u16(self.pc + 2)?;

                let (result, carry_occurred) = val1.overflowing_sub(val2);

                self.update_flags(result, Some(carry_occurred));
                self.pc += 4;
            }

            // JMP lit (0x50)
            0x50 => {
                let addr = rom.read_u16(self.pc + 1)?;
                self.pc = addr;
            }
            // JMP reg (0x51)
            0x51 => {
                let source_idx = self.get_register_index(self.pc + 1, rom)?;
                let addr = self.registers[source_idx];
                self.pc = addr;
            }
            // JZ lit (0x52)
            0x52 => {
                let addr = rom.read_u16(self.pc + 1)?;
                if self.is_flag_set(FLAG_ZERO) {
                    self.pc = addr;
                } else {
                    self.pc += 3
                }
            }
            // JZ reg (0x53)
            0x53 => {
                let source_idx = self.get_register_index(self.pc + 1, rom)?;
                let addr = self.registers[source_idx];
                if self.is_flag_set(FLAG_ZERO) {
                    self.pc = addr;
                } else {
                    self.pc += 2
                }
            }
            // JNZ lit (0x54)
            0x54 => {
                let addr = rom.read_u16(self.pc + 1)?;
                if !self.is_flag_set(FLAG_ZERO) {
                    self.pc = addr;
                } else {
                    self.pc += 3
                }
            }
            // JNZ reg (0x55)
            0x55 => {
                let source_idx = self.get_register_index(self.pc + 1, rom)?;
                let addr = self.registers[source_idx];
                if !self.is_flag_set(FLAG_ZERO) {
                    self.pc = addr;
                } else {
                    self.pc += 2
                }
            }
            // JN lit (0x56)
            0x56 => {
                let addr = rom.read_u16(self.pc + 1)?;
                if self.is_flag_set(FLAG_NEGATIVE) {
                    self.pc = addr;
                } else {
                    self.pc += 3
                }
            }
            // JN reg (0x57)
            0x57 => {
                let source_idx = self.get_register_index(self.pc + 1, rom)?;
                let addr = self.registers[source_idx];
                if self.is_flag_set(FLAG_NEGATIVE) {
                    self.pc = addr;
                } else {
                    self.pc += 2
                }
            }
            // JNN lit (0x58)
            0x58 => {
                let addr = rom.read_u16(self.pc + 1)?;
                if !self.is_flag_set(FLAG_NEGATIVE) {
                    self.pc = addr;
                } else {
                    self.pc += 3
                }
            }
            // JNN reg (0x59)
            0x59 => {
                let source_idx = self.get_register_index(self.pc + 1, rom)?;
                let addr = self.registers[source_idx];
                if !self.is_flag_set(FLAG_NEGATIVE) {
                    self.pc = addr;
                } else {
                    self.pc += 2
                }
            }
            // JC lit (0x5A)
            0x5A => {
                let addr = rom.read_u16(self.pc + 1)?;
                if self.is_flag_set(FLAG_CARRY) {
                    self.pc = addr;
                } else {
                    self.pc += 3
                }
            }
            // JC reg (0x5B)
            0x5B => {
                let source_idx = self.get_register_index(self.pc + 1, rom)?;
                let addr = self.registers[source_idx];
                if self.is_flag_set(FLAG_CARRY) {
                    self.pc = addr;
                } else {
                    self.pc += 2
                }
            }
            // JNC lit (0xC8)
            0x5C => {
                let addr = rom.read_u16(self.pc + 1)?;
                if !self.is_flag_set(FLAG_CARRY) {
                    self.pc = addr;
                } else {
                    self.pc += 3
                }
            }
            // JNC reg (0xD9)
            0x5D => {
                let source_idx = self.get_register_index(self.pc + 1, rom)?;
                let addr = self.registers[source_idx];
                if !self.is_flag_set(FLAG_CARRY) {
                    self.pc = addr;
                } else {
                    self.pc += 2
                }
            }

            // CALL lit (0x60)
            0x60 => {
                let target_addr = rom.read_u16(self.pc + 1)?;
                let return_addr = self.pc + 3;
                self.stack.push(return_addr);
                self.pc = target_addr;
            }
            // RET (0x61)
            0x61 => {
                let return_addr = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                self.pc = return_addr;
            }
            _ => {
                return Err(VMError::InvalidOpcode(opcode));
            }
        }
        // println!("OP {:02X} Reg {:?}", opcode, self.registers);
        Ok(())
    }
}

impl Display for CupanaMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CupanaMachine {{\n  registers: {:?},\n  stack: {:?},\n  pc: {},\n  flags: {:08b}\n}}",
            self.registers, self.stack, self.pc, self.flags
        )
    }
}
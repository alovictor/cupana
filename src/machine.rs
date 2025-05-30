use std::fmt::Display;

use crate::{
    error::VMError,
    memory::{Memory, MemoryBus, STACK_END},
};

const NUM_REGISTERS: usize = 0x10; // 16 registradores
const FLAG_ZERO: u8 = 0b0000_0001;
const FLAG_CARRY: u8 = 0b0000_0010;
const FLAG_NEGATIVE: u8 = 0b0000_0100;
const FLAG_INTERRUPT_DISABLED: u8 = 0b0000_1000;
const FLAG_INTERRUPT_REQUEST_PENDING: u8 = 0b0001_0000;
const FLAG_HALT: u8 = 0b1000_0000;

const NON_MASKABLE_INTERRUPT_REQUEST_VECTOR: u16 = 0x7FFA;
const RESET_VECTOR: u16 = 0x7FFC;
const INTERRUPT_REQUEST_VECTOR: u16 = 0x7FFE;

pub struct CupanaMachine {
    registers: [u16; NUM_REGISTERS],
    pc: u16,
    sp: u16,
    flags: u8,
}

impl CupanaMachine {
    pub fn new() -> Self {
        Self {
            registers: [0; NUM_REGISTERS],
            pc: 0,
            sp: 0,
            flags: 0,
        }
    }

    pub fn reset(&mut self, mem_bus: &mut MemoryBus) -> Result<(), VMError> {
        self.registers = [0; NUM_REGISTERS];
        self.pc = mem_bus.read_u16(RESET_VECTOR)?;
        self.sp = STACK_END - 1;
        self.flags = FLAG_INTERRUPT_DISABLED;
        Ok(())
    }

    pub fn has_halted(&self) -> bool {
        self.flags & FLAG_HALT != 0
    }

    pub fn request_interrupt(&mut self) {
        self.update_flag(FLAG_INTERRUPT_REQUEST_PENDING, true);
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

    fn push_u8(&mut self, mem_bus: &mut MemoryBus, value: u8) -> Result<(), VMError> {
        if self.sp < crate::memory::STACK_BASE {
            return Err(VMError::StackOverflow);
        }
        mem_bus.write_u8(self.sp, value)?;
        self.sp -= 1;
        Ok(())
    }

    fn pop_u8(&mut self, mem_bus: &mut MemoryBus) -> Result<u8, VMError> {
        if self.sp >= STACK_END -1 { 
             return Err(VMError::StackUnderflow);
        }
        self.sp += 1;
        Ok(mem_bus.read_u8(self.sp)?)
    }

    fn push_u16(&mut self, mem_bus: &mut MemoryBus, value: u16) -> Result<(), VMError> {
        if self.sp < crate::memory::STACK_BASE + 1 { // Need 2 bytes
            return Err(VMError::StackOverflow);
        }
        // Push High Byte
        self.push_u8(mem_bus, (value >> 8) as u8)?;
        // Push Low Byte
        self.push_u8(mem_bus, (value & 0xFF) as u8)?;
        Ok(())
    }

    fn pop_u16(&mut self, mem_bus: &mut MemoryBus) -> Result<u16, VMError> {
        if self.sp >= STACK_END - 2 { // Check before increment to prevent wrapping issues. Need 2 bytes.
             return Err(VMError::StackUnderflow);
        }
        let lo = self.pop_u8(mem_bus)? as u16;
        let hi = self.pop_u8(mem_bus)? as u16;
        Ok((hi << 8) | lo)
    }

    fn get_register_index(&self, pc: u16, mem_bus: &mut MemoryBus) -> Result<usize, VMError> {
        let reg = mem_bus.read_u8(pc)?;
        if reg < NUM_REGISTERS as u8 {
            Ok(reg as usize)
        } else {
            Err(VMError::InvalidRegister(reg))
        }
    }

    pub fn step(&mut self, mem_bus: &mut MemoryBus) -> Result<(), VMError> {

        if self.is_flag_set(FLAG_INTERRUPT_REQUEST_PENDING) && !self.is_flag_set(FLAG_INTERRUPT_DISABLED) {
            let return_pc = self.pc;        // Save current PC for pushing
            let return_flags = self.flags;   // Save current flags for pushing

            // 1. Automatically disable further maskable interrupts in the CPU's current flags
            self.update_flag(FLAG_INTERRUPT_DISABLED, true);

            // 2. Clear the pending interrupt request
            self.update_flag(FLAG_INTERRUPT_REQUEST_PENDING, false);

            // 3. Push the *original* (return) PC onto the stack
            self.push_u16(mem_bus, return_pc)?;

            // 4. Push the *original* (return) flags onto the stack
            self.push_u8(mem_bus, return_flags)?;

            // 5. Load PC with the ISR address from the interrupt vector table
            // INTERRUPT_REQUEST_VECTOR (e.g., 0x7FFE) holds the ISR's actual starting address
            let isr_address = mem_bus.read_u16(INTERRUPT_REQUEST_VECTOR)?;
            self.pc = isr_address;

            return Ok(()); // Skip fetching the normal next instruction
        }

        let opcode = mem_bus.read_u8(self.pc)?;
        match opcode {
            // NOP (0x00)
            0x00 => {
                self.pc += 1;
            }
            // HLT (0x01)
            0x01 => {
                self.update_flag(FLAG_HALT, true);
                self.pc += 1;
            }
            // MOV reg reg (0x10)
            0x10 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let source_idx = self.get_register_index(self.pc + 2, mem_bus)?;
                self.registers[dest_idx] = self.registers[source_idx];
                self.pc += 3;
            }
            // MOV reg lit (0x11)
            0x11 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let source = mem_bus.read_u16(self.pc + 2)?;
                self.registers[dest_idx] = source;
                self.pc += 4;
            }
            // MOV reg reg* (0x12)
            0x12 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let source_idx = self.get_register_index(self.pc + 2, mem_bus)?;
                self.registers[dest_idx] = mem_bus.read_u16(self.registers[source_idx])?;
                self.pc += 3;
            }
            // MOV mem reg (0x13)
            0x13 => {
                let dest = mem_bus.read_u16(self.pc + 1)?;
                let source_idx = self.get_register_index(self.pc + 3, mem_bus)?;
                mem_bus.write_u16(dest, self.registers[source_idx])?;
                self.pc += 4;
            }
            // MOV mem lit (0x14)
            0x14 => {
                let dest = mem_bus.read_u16(self.pc + 1)?;
                let source = mem_bus.read_u16(self.pc + 3)?;
                mem_bus.write_u16(dest, source)?;
                self.pc += 5;
            }
            // MOV reg* reg (0x15)
            0x15 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let source_idx = self.get_register_index(self.pc + 2, mem_bus)?;
                mem_bus.write_u16(self.registers[dest_idx], self.registers[source_idx])?;
                self.pc += 3;
            }
            // MOV reg* lit (0x16)
            0x16 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let source = mem_bus.read_u16(self.pc + 2)?;
                mem_bus.write_u16(self.registers[dest_idx], source)?;
                self.pc += 4;
            }
            // PHR Reg (0x17)
            0x17 => {
                let src_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                self.push_u16(mem_bus, self.registers[src_idx])?;
                self.pc += 2;
            }
            // PLR Reg (0x18)
            0x18 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                self.registers[dest_idx] = self.pop_u16(mem_bus)?;
                self.pc += 2;
            }
            // ADD reg reg (0x20)
            0x20 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let source2_idx = self.get_register_index(self.pc + 2, mem_bus)?;

                let val1 = self.registers[dest_idx];
                let val2 = self.registers[source2_idx];

                let (result, carry_occurred) = val1.overflowing_add(val2);
                self.registers[dest_idx] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 3;
            }
            // ADD reg lit (0x21)
            0x21 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let val1 = self.registers[dest_idx];
                let val2 = mem_bus.read_u16(self.pc + 2)?;

                let (result, carry_occurred) = val1.overflowing_add(val2);
                self.registers[dest_idx] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 4;
            }
            // SUB reg reg (0x22)
            0x22 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let source2_idx = self.get_register_index(self.pc + 2, mem_bus)?;

                let val1 = self.registers[dest_idx];
                let val2 = self.registers[source2_idx];

                let (result, carry_occurred) = val1.overflowing_sub(val2);
                self.registers[dest_idx] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 3;
            }
            // SUB reg lit (0x23)
            0x23 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let val1 = self.registers[dest_idx];
                let val2 = mem_bus.read_u16(self.pc + 2)?;

                let (result, carry_occurred) = val1.overflowing_sub(val2);
                self.registers[dest_idx] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 4;
            }
            // MUL reg reg (0x24)
            0x24 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let source2_idx = self.get_register_index(self.pc + 2, mem_bus)?;

                let val1 = self.registers[dest_idx];
                let val2 = self.registers[source2_idx];

                let (result, carry_occurred) = val1.overflowing_mul(val2);
                self.registers[dest_idx] = result;
                self.update_flags(result, Some(carry_occurred));

                self.pc += 3;
            }
            // MUL reg lit (0x25)
            0x25 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let val1 = self.registers[dest_idx];
                let val2 = mem_bus.read_u16(self.pc + 2)?;

                let (result, carry_occurred) = val1.overflowing_mul(val2);
                self.registers[dest_idx] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 4;
            }
            // DIV reg reg (0x26)
            0x26 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let source2_idx = self.get_register_index(self.pc + 2, mem_bus)?;

                let val1 = self.registers[dest_idx];
                let val2 = self.registers[source2_idx];

                if val2 == 0 {
                    return Err(VMError::DivideByZero); // Ou lidar via exceção se implementado
                } else {
                    let (result, carry_occurred) = val1.overflowing_div(val2);
                    self.registers[dest_idx] = result;

                    self.update_flags(result, Some(carry_occurred));
                    self.pc += 3;
                }
            }
            // DIV reg lit (0x27)
            0x27 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let val1 = self.registers[dest_idx];
                let val2 = mem_bus.read_u16(self.pc + 2)?;

                if val2 == 0 {
                    return Err(VMError::DivideByZero); // Ou lidar via exceção se implementado
                } else {
                    let (result, carry_occurred) = val1.overflowing_div(val2);
                    self.registers[dest_idx] = result;

                    self.update_flags(result, Some(carry_occurred));
                    self.pc += 4;
                }
            }
            // MOD reg reg (0x28)
            0x28 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let source2_idx = self.get_register_index(self.pc + 2, mem_bus)?;

                let val1 = self.registers[dest_idx];
                let val2 = self.registers[source2_idx];

                if val2 == 0 {
                    return Err(VMError::DivideByZero); // Ou lidar via exceção se implementado
                } else {
                    let (result, carry_occurred) = val1.overflowing_rem(val2);
                    self.registers[dest_idx] = result;

                    self.update_flags(result, Some(carry_occurred));
                    self.pc += 3;
                }
            }
            // MOD reg lit (0x29)
            0x29 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let val1 = self.registers[dest_idx];
                let val2 = mem_bus.read_u16(self.pc + 2)?;

                if val2 == 0 {
                    return Err(VMError::DivideByZero); // Ou lidar via exceção se implementado
                } else {
                    let (result, carry_occurred) = val1.overflowing_rem(val2);
                    self.registers[dest_idx] = result;

                    self.update_flag(FLAG_ZERO, result == 0);
                    self.update_flag(FLAG_NEGATIVE, (result & 0x8000) != 0);
                    self.update_flag(FLAG_CARRY, carry_occurred);
                    self.pc += 4;
                }
            }
            // INC (0x2A)
            0x2A => {
                let idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let val1 = self.registers[idx];

                let (result, carry_occurred) = val1.overflowing_add(1);
                self.registers[idx] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 2;
            }
            // DEC (0x2B)
            0x2B => {
                let idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let val1 = self.registers[idx];

                let (result, carry_occurred) = val1.overflowing_sub(1);
                self.registers[idx] = result;

                self.update_flags(result, Some(carry_occurred));
                self.pc += 2;
            }

            // AND reg reg (0x30)
            0x30 => {
                let source1_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let source2_idx = self.get_register_index(self.pc + 2, mem_bus)?;

                let val1 = self.registers[source1_idx];
                let val2 = self.registers[source2_idx];

                let result = val1 & val2;
                self.registers[source1_idx] = result;

                self.update_flags(result, None);
                self.pc += 3;
            }
            // AND reg lit (0x31)
            0x31 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let val1 = self.registers[dest_idx];
                let val2 = mem_bus.read_u16(self.pc + 2)?;

                let result = val1 & val2;
                self.registers[dest_idx] = result;

                self.update_flags(result, None);
                self.pc += 3;
            }
            // OR reg reg (0x32)
            0x32 => {
                let source1_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let source2_idx = self.get_register_index(self.pc + 2, mem_bus)?;

                let val1 = self.registers[source1_idx];
                let val2 = self.registers[source2_idx];

                let result = val1 | val2;
                self.registers[source1_idx] = result;

                self.update_flags(result, None);
                self.pc += 3;
            }
            // OR reg lit (0x33)
            0x33 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let val1 = self.registers[dest_idx];
                let val2 = mem_bus.read_u16(self.pc + 2)?;

                let result = val1 | val2;
                self.registers[dest_idx] = result;

                self.update_flags(result, None);
                self.pc += 3;
            }
            // XOR reg reg (0x34)
            0x34 => {
                let source1_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let source2_idx = self.get_register_index(self.pc + 2, mem_bus)?;

                let val1 = self.registers[source1_idx];
                let val2 = self.registers[source2_idx];

                let result = val1 ^ val2;
                self.registers[source1_idx] = result;

                self.update_flags(result, None);
                self.pc += 3;
            }
            // XOR reg lit (0x35)
            0x35 => {
                let dest_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let val1 = self.registers[dest_idx];
                let val2 = mem_bus.read_u16(self.pc + 2)?;

                let result = val1 ^ val2;
                self.registers[dest_idx] = result;

                self.update_flags(result, None);
                self.pc += 3;
            }
            // NOT reg (0x36)
            0x36 => {
                let source1_idx = self.get_register_index(self.pc + 1, mem_bus)?;

                let val1 = self.registers[source1_idx];

                let result = !val1;
                self.registers[source1_idx] = result;

                self.update_flags(result, None);
                self.pc += 2;
            }

            // CMP reg reg (0x40)
            0x40 => {
                let source1_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let source2_idx = self.get_register_index(self.pc + 2, mem_bus)?;

                let val1 = self.registers[source1_idx];
                let val2 = self.registers[source2_idx];

                let (result, carry_occurred) = val1.overflowing_sub(val2);

                self.update_flags(result, Some(carry_occurred));
                self.pc += 3;
            }
            // CMP reg lit (0x41)
            0x41 => {
                let source1_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let val1 = self.registers[source1_idx];
                let val2 = mem_bus.read_u16(self.pc + 2)?;

                let (result, carry_occurred) = val1.overflowing_sub(val2);

                self.update_flags(result, Some(carry_occurred));
                self.pc += 4;
            }

            // JMP lit (0x50)
            0x50 => {
                let addr = mem_bus.read_u16(self.pc + 1)?;
                self.pc = addr;
            }
            // JMP reg (0x51)
            0x51 => {
                let source_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let addr = self.registers[source_idx];
                self.pc = addr;
            }
            // JZ lit (0x52)
            0x52 => {
                let addr = mem_bus.read_u16(self.pc + 1)?;
                if self.is_flag_set(FLAG_ZERO) {
                    self.pc = addr;
                } else {
                    self.pc += 3
                }
            }
            // JZ reg (0x53)
            0x53 => {
                let source_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let addr = self.registers[source_idx];
                if self.is_flag_set(FLAG_ZERO) {
                    self.pc = addr;
                } else {
                    self.pc += 2
                }
            }
            // JNZ lit (0x54)
            0x54 => {
                let addr = mem_bus.read_u16(self.pc + 1)?;
                if !self.is_flag_set(FLAG_ZERO) {
                    self.pc = addr;
                } else {
                    self.pc += 3
                }
            }
            // JNZ reg (0x55)
            0x55 => {
                let source_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let addr = self.registers[source_idx];
                if !self.is_flag_set(FLAG_ZERO) {
                    self.pc = addr;
                } else {
                    self.pc += 2
                }
            }
            // JN lit (0x56)
            0x56 => {
                let addr = mem_bus.read_u16(self.pc + 1)?;
                if self.is_flag_set(FLAG_NEGATIVE) {
                    self.pc = addr;
                } else {
                    self.pc += 3
                }
            }
            // JN reg (0x57)
            0x57 => {
                let source_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let addr = self.registers[source_idx];
                if self.is_flag_set(FLAG_NEGATIVE) {
                    self.pc = addr;
                } else {
                    self.pc += 2
                }
            }
            // JNN lit (0x58)
            0x58 => {
                let addr = mem_bus.read_u16(self.pc + 1)?;
                if !self.is_flag_set(FLAG_NEGATIVE) {
                    self.pc = addr;
                } else {
                    self.pc += 3
                }
            }
            // JNN reg (0x59)
            0x59 => {
                let source_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let addr = self.registers[source_idx];
                if !self.is_flag_set(FLAG_NEGATIVE) {
                    self.pc = addr;
                } else {
                    self.pc += 2
                }
            }
            // JC lit (0x5A)
            0x5A => {
                let addr = mem_bus.read_u16(self.pc + 1)?;
                if self.is_flag_set(FLAG_CARRY) {
                    self.pc = addr;
                } else {
                    self.pc += 3
                }
            }
            // JC reg (0x5B)
            0x5B => {
                let source_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let addr = self.registers[source_idx];
                if self.is_flag_set(FLAG_CARRY) {
                    self.pc = addr;
                } else {
                    self.pc += 2
                }
            }
            // JNC lit (0xC8)
            0x5C => {
                let addr = mem_bus.read_u16(self.pc + 1)?;
                if !self.is_flag_set(FLAG_CARRY) {
                    self.pc = addr;
                } else {
                    self.pc += 3
                }
            }
            // JNC reg (0xD9)
            0x5D => {
                let source_idx = self.get_register_index(self.pc + 1, mem_bus)?;
                let addr = self.registers[source_idx];
                if !self.is_flag_set(FLAG_CARRY) {
                    self.pc = addr;
                } else {
                    self.pc += 2
                }
            }

            // JSB lit (0x5E)
            0x5E => {
                let target_addr = mem_bus.read_u16(self.pc + 1)?;
                let return_addr = self.pc + 3;
                self.push_u16(mem_bus, return_addr)?;
                self.pc = target_addr;
            }
            // RSB (0x5F)
            0x5F => {
                self.pc = self.pop_u16(mem_bus)?;
            }
            // CLI (0x60)
            0x60 => { 
                self.update_flag(FLAG_INTERRUPT_DISABLED, false);
                self.pc += 1;
            }
            // SEI (0x61)
            0x61 => { 
                self.update_flag(FLAG_INTERRUPT_DISABLED, true);
                self.pc += 1;
            }
            // RSI (0x62)
            0x62 => {
                self.flags = self.pop_u8(mem_bus)?;
                // Pop return address
                self.pc = self.pop_u16(mem_bus)?;
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
        writeln!(f, "CupanaMachine {{")?;
        writeln!(f, "  PC: 0x{:04X}, SP: 0x{:04X}", self.pc, self.sp)?;
        writeln!(f, "  Flags: {:08b} (Z:{} N:{} C:{} ID:{})",
            self.flags,
            self.is_flag_set(FLAG_ZERO) as u8,
            self.is_flag_set(FLAG_NEGATIVE) as u8,
            self.is_flag_set(FLAG_CARRY) as u8,
            self.is_flag_set(FLAG_INTERRUPT_DISABLED) as u8,
        )?;
        for i in 0..NUM_REGISTERS {
            if i % 4 == 0 {
                write!(f, "  ")?;
            }
            write!(f, "R{:02}: 0x{:04X}{}", i, self.registers[i], if (i + 1) % 4 == 0 {"\n"} else {" "})?;
        }
        if NUM_REGISTERS % 4 != 0 { writeln!(f)?; }
        write!(f, "}}")
    }
}
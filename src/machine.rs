use crate::memory::{Memory, RAM_BASE, ROM_BASE, STACK_BASE};
const PC: usize = 14;
const SP: usize = 15;

#[derive(Debug)]
enum Opcode {
    NOP,
    HLT,
    MOV,
    PHR,
    PLR,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    INC,
    DEC,
    AND,
    OR,
    XOR,
    SHL,
    SHR,
    NOT,
    CMP,
    JMP,
    JPC,
    JSB,
    RSB,
    CLI,
    SEI,
    RSI,
    NONE,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Opcode::NOP,
            0x01 => Opcode::HLT,
            0x02 => Opcode::MOV,
            0x03 => Opcode::PHR,
            0x04 => Opcode::PLR,
            0x05 => Opcode::ADD,
            0x06 => Opcode::SUB,
            0x07 => Opcode::MUL,
            0x08 => Opcode::DIV,
            0x09 => Opcode::MOD,
            0x0A => Opcode::INC,
            0x0B => Opcode::DEC,
            0x0C => Opcode::AND,
            0x0D => Opcode::OR,
            0x0E => Opcode::XOR,
            0x0F => Opcode::SHL,
            0x10 => Opcode::SHR,
            0x11 => Opcode::NOT,
            0x12 => Opcode::CMP,
            0x13 => Opcode::JMP,
            0x14 => Opcode::JPC,
            0x15 => Opcode::JSB,
            0x16 => Opcode::RSB,
            0x17 => Opcode::CLI,
            0x18 => Opcode::SEI,
            0x19 => Opcode::RSI,
            _ => Opcode::NONE,
        }
    }
}

pub enum Flag {
    Zero = 0x0001,
    Negative = 0x0002,
    Overflow = 0x0004,
    InterruptDisabled = 0x0008,
    InterruptPending = 0x0010,
    Halt = 0x0080,
}

fn extract_registers_from_byte(byte: u8) -> (u8, u8) {
    let reg_a = (byte >> 4) & 0b1111;
    let reg_b = byte & 0b1111;
    (reg_a, reg_b)
}

pub struct Machine {
    registers: [u16; 16],
    flags: u16,
}

impl Machine {
    pub fn new() -> Self {
        let mut registers = [0; 16];
        registers[PC] = ROM_BASE;
        registers[SP] = STACK_BASE;

        Machine {
            registers,
            flags: 0,
        }
    }

    pub fn reset(&mut self) {
        self.registers = [0; 16];
        self.flags = 0;
    }

    pub fn halted(&self) -> bool {
        self.get_flag(Flag::Halt)
    }

    fn get_flag(&self, flag: Flag) -> bool {
        (self.flags & flag as u16) != 0
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.flags |= flag as u16;
        } else {
            self.flags &= !(flag as u16);
        }
    }

    fn fetch_u8(&mut self, mem: &Memory) -> u8 {
        let addr = self.registers[PC];
        self.registers[PC] = self.registers[PC].wrapping_add(1);
        mem.read_u8(addr)
    }

    fn fetch_u16(&mut self, mem: &Memory) -> u16 {
        let addr = self.registers[PC];
        self.registers[PC] = self.registers[PC].wrapping_add(2);
        mem.read_u16(addr)
    }

    fn push_u8(&mut self, mem: &mut Memory, value: u8) -> Result<(), String> {
        mem.write_u8(self.registers[SP], value);
        self.registers[SP] = self.registers[SP].wrapping_add(1);
        Ok(())
    }

    fn push_u16(&mut self, mem: &mut Memory, value: u16) -> Result<(), String> {
        mem.write_u16(self.registers[SP], value);
        self.registers[SP] = self.registers[SP].wrapping_add(2);
        Ok(())
    }

    fn pull_u8(&mut self, mem: &mut Memory) -> u8 {
        let value = mem.read_u8(self.registers[SP]);
        self.registers[SP] = self.registers[SP].wrapping_sub(1);
        value
    }

    fn pull_u16(&mut self, mem: &mut Memory) -> u16 {
        self.registers[SP] = self.registers[SP].wrapping_sub(2);
        let value = mem.read_u16(self.registers[SP]);
        value
    }

    fn update_flags(&mut self, (result, overflow): (u16, bool)) {
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::Negative, (result & 0x8000) != 0);
        self.set_flag(Flag::Overflow, overflow);
    }

    fn print_state(&self, mem: &Memory) {
        println!("------------------------");
        println!(
            "  PC: {:04X}   SP: {:04X}",
            self.registers[PC], self.registers[SP]
        );
        println!("  FLAGS: {:08b} ", self.flags as u8);
        println!("REGISTRADORES: ");
        let offset = self.registers.len() / 2;
        for idx in (0..offset) {
            println!(
                "  R{:02}: {:04X}   R{:02}: {:04X}",
                idx,
                self.registers[idx],
                idx + offset,
                self.registers[idx + offset],
            );
        }

        println!("{}", mem);
        println!("------------------------");
    }

    pub fn step(&mut self, mem: &mut Memory) {
        let byte = self.fetch_u8(mem);
        let opcode = Opcode::from(byte >> 3);
        let b = (byte >> 2) & 1;
        let mode = byte & 0b11;

        match opcode {
            Opcode::NOP => {
                println!("NOP");
            }
            Opcode::HLT => {
                println!("HLT");
                self.set_flag(Flag::Halt, true)
            }
            Opcode::MOV => match b {
                0 => match mode {
                    0 => {
                        let (dest, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("MOV R{}, R{}", dest, orig);
                        let value = self.registers[orig as usize];
                        self.registers[dest as usize] = value;
                    }
                    1 => {
                        let reg = self.fetch_u8(mem);
                        let value = self.fetch_u16(mem);
                        println!("MOV R{}, {}", reg, value);
                        self.registers[reg as usize] = value;
                    }
                    2 => {
                        let (desti, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("MOV R{}*, R{}", desti, orig);
                        let addr = self.registers[desti as usize];
                        let value = self.registers[orig as usize];
                        mem.write_u16(addr, value);
                    }
                    3 => {
                        let (dest, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("MOV R{}, R{}*", dest, orig);
                        let addr = self.registers[orig as usize];
                        let value = mem.read_u16(addr);
                        self.registers[dest as usize] = value;
                    }
                    _ => unreachable!(),
                },
                1 => match mode {
                    0 => {
                        let (dest, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("MOVB R{}, R{}", dest, orig);
                        let value = self.registers[orig as usize] as u8;
                        self.registers[dest as usize] = value as u16;
                    }
                    1 => {
                        let reg = self.fetch_u8(mem);
                        let value = self.fetch_u8(mem);
                        println!("MOVB R{}, {}", reg, value);
                        self.registers[reg as usize] = value as u16;
                    }
                    2 => {
                        let (desti, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("MOVB R{}*, R{}", desti, orig);
                        let addr = self.registers[desti as usize];
                        let value = self.registers[orig as usize] as u8;
                        mem.write_u8(addr, value);
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            Opcode::PHR => {
                let orig = self.fetch_u8(mem);
                println!("PHR R{}", orig);
                self.push_u16(mem, self.registers[orig as usize])
                    .expect("Erro no push_16");
            }
            Opcode::PLR => {
                let dest = self.fetch_u8(mem);
                println!("PLR R{}", dest);
                self.registers[dest as usize] = self.pull_u16(mem);
            }
            Opcode::ADD => match b {
                // Short
                0 => match mode {
                    0 => {
                        let (dest, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("ADD R{}, R{}", dest, orig);
                        let value_dest = self.registers[dest as usize];
                        let value_orig = self.registers[orig as usize];
                        let result = value_dest.overflowing_add(value_orig);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    1 => {
                        let dest = self.fetch_u8(mem);
                        let value_dest = self.registers[dest as usize];
                        let value = self.fetch_u16(mem);
                        println!("ADD R{}, {}", dest, value);
                        let result = value_dest.overflowing_add(value);
                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    _ => {
                        unreachable!()
                    }
                },
                // Byte
                1 => match mode {
                    0 => {
                        let (dest, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("ADD R{}, R{}", dest, orig);
                        let value_dest = self.registers[dest as usize] & 0xFF;
                        let value_orig = self.registers[orig as usize] & 0xFF;
                        let result = value_dest.overflowing_add(value_orig);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    1 => {
                        let dest = self.fetch_u8(mem);
                        let value_dest = self.registers[dest as usize] & 0xFF;
                        let value = self.fetch_u8(mem) as u16;
                        println!("ADD R{}, {}", dest, value);
                        let result = value_dest.overflowing_add(value);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    _ => {
                        unreachable!()
                    }
                },
                _ => {
                    unreachable!()
                }
            },
            Opcode::SUB => match b {
                // Short
                0 => match mode {
                    0 => {
                        let (dest, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("SUB R{}, R{}", dest, orig);
                        let value_dest = self.registers[dest as usize];
                        let value_orig = self.registers[orig as usize];
                        let result = value_dest.overflowing_sub(value_orig);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    1 => {
                        let dest = self.fetch_u8(mem);
                        let value_dest = self.registers[dest as usize];
                        let value = self.fetch_u16(mem);
                        println!("SUB R{}, {}", dest, value);
                        let result = value_dest.overflowing_sub(value);
                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    _ => {
                        unreachable!()
                    }
                },
                // Byte
                1 => match mode {
                    0 => {
                        let (dest, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("SUB R{}, R{}", dest, orig);
                        let value_dest = self.registers[dest as usize] & 0xFF;
                        let value_orig = self.registers[orig as usize] & 0xFF;
                        let result = value_dest.overflowing_sub(value_orig);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    1 => {
                        let dest = self.fetch_u8(mem);
                        let value_dest = self.registers[dest as usize] & 0xFF;
                        let value = self.fetch_u8(mem) as u16;
                        println!("SUB R{}, {}", dest, value);
                        let result = value_dest.overflowing_sub(value);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    _ => {
                        unreachable!()
                    }
                },
                _ => {
                    unreachable!()
                }
            },
            Opcode::MUL => match b {
                // Short
                0 => match mode {
                    0 => {
                        let (dest, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("MUL R{}, R{}", dest, orig);
                        let value_dest = self.registers[dest as usize];
                        let value_orig = self.registers[orig as usize];
                        let result = value_dest.overflowing_mul(value_orig);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    1 => {
                        let dest = self.fetch_u8(mem);
                        let value_dest = self.registers[dest as usize];
                        let value = self.fetch_u16(mem);
                        println!("MUL R{}, {}", dest, value);
                        let result = value_dest.overflowing_mul(value);
                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    _ => {
                        unreachable!()
                    }
                },
                // Byte
                1 => match mode {
                    0 => {
                        let (dest, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("MUL R{}, R{}", dest, orig);
                        let value_dest = self.registers[dest as usize] & 0xFF;
                        let value_orig = self.registers[orig as usize] & 0xFF;
                        let result = value_dest.overflowing_mul(value_orig);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    1 => {
                        let dest = self.fetch_u8(mem);
                        let value_dest = self.registers[dest as usize] & 0xFF;
                        let value = self.fetch_u8(mem) as u16;
                        println!("MUL R{}, {}", dest, value);
                        let result = value_dest.overflowing_mul(value);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    _ => {
                        unreachable!()
                    }
                },
                _ => {
                    unreachable!()
                }
            },
            Opcode::DIV => match b {
                // Short
                0 => match mode {
                    0 => {
                        let (dest, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("DIV R{}, R{}", dest, orig);
                        let value_dest = self.registers[dest as usize];
                        let value_orig = self.registers[orig as usize];
                        let result = value_dest.overflowing_div(value_orig);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    1 => {
                        let dest = self.fetch_u8(mem);
                        let value_dest = self.registers[dest as usize];
                        let value = self.fetch_u16(mem);
                        println!("DIV R{}, {}", dest, value);
                        let result = value_dest.overflowing_div(value);
                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    _ => {
                        unreachable!()
                    }
                },
                // Byte
                1 => match mode {
                    0 => {
                        let (dest, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("DIV R{}, R{}", dest, orig);
                        let value_dest = self.registers[dest as usize] & 0xFF;
                        let value_orig = self.registers[orig as usize] & 0xFF;
                        let result = value_dest.overflowing_div(value_orig);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    1 => {
                        let dest = self.fetch_u8(mem);
                        let value_dest = self.registers[dest as usize] & 0xFF;
                        let value = self.fetch_u8(mem) as u16;
                        println!("DIV R{}, {}", dest, value);
                        let result = value_dest.overflowing_div(value);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    _ => {
                        unreachable!()
                    }
                },
                _ => {
                    unreachable!()
                }
            },
            Opcode::MOD => match b {
                // Short
                0 => match mode {
                    0 => {
                        let (dest, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("MOD R{}, R{}", dest, orig);
                        let value_dest = self.registers[dest as usize];
                        let value_orig = self.registers[orig as usize];
                        let result = value_dest.overflowing_rem(value_orig);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    1 => {
                        let dest = self.fetch_u8(mem);
                        let value_dest = self.registers[dest as usize];
                        let value = self.fetch_u16(mem);
                        println!("MOD R{}, {}", dest, value);
                        let result = value_dest.overflowing_rem(value);
                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    _ => {
                        unreachable!()
                    }
                },
                // Byte
                1 => match mode {
                    0 => {
                        let (dest, orig) = extract_registers_from_byte(self.fetch_u8(mem));
                        println!("MOD R{}, R{}", dest, orig);
                        let value_dest = self.registers[dest as usize] & 0xFF;
                        let value_orig = self.registers[orig as usize] & 0xFF;
                        let result = value_dest.overflowing_rem(value_orig);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    1 => {
                        let dest = self.fetch_u8(mem);
                        let value_dest = self.registers[dest as usize] & 0xFF;
                        let value = self.fetch_u8(mem) as u16;
                        println!("MOD R{}, {}", dest, value);
                        let result = value_dest.overflowing_rem(value);

                        self.update_flags(result);
                        self.registers[dest as usize] = result.0;
                    }
                    _ => {
                        unreachable!()
                    }
                },
                _ => {
                    unreachable!()
                }
            },
            Opcode::INC => match b {
                0 => {
                    let dest = self.fetch_u8(mem);
                    println!("INC R{}", dest);
                    let value_dest = self.registers[dest as usize];
                    let result = value_dest.overflowing_add(1);

                    self.update_flags(result);
                    self.registers[dest as usize] = result.0;
                }
                1 => {
                    let dest = self.fetch_u8(mem);
                    println!("INC R{}", dest);
                    let value_dest = self.registers[dest as usize] as u8;
                    let result = value_dest.overflowing_add(1);

                    self.update_flags((result.0 as u16, result.1));
                    self.registers[dest as usize] = result.0 as u16;
                }
                _ => {
                    unreachable!()
                }
            },
            Opcode::DEC => match b {
                0 => {
                    let dest = self.fetch_u8(mem);
                    println!("DEC R{}", dest);
                    let value_dest = self.registers[dest as usize];
                    let result = value_dest.overflowing_sub(1);

                    self.update_flags(result);
                    self.registers[dest as usize] = result.0;
                }
                1 => {
                    let dest = self.fetch_u8(mem);
                    println!("DEC R{}", dest);
                    let value_dest = self.registers[dest as usize] as u8;
                    let result = value_dest.overflowing_sub(1);

                    self.update_flags((result.0 as u16, result.1));
                    self.registers[dest as usize] = result.0 as u16;
                }
                _ => {
                    unreachable!()
                }
            },
            _ => {
                panic!("Unimplemented opcode: {:?}", opcode);
            }
        }
        // self.print_state(mem);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Memory;

    #[test]
    fn test_reset() {
        let mut machine = Machine::new();
        machine.registers[0] = 0x1234;
        machine.flags = 0x56;
        machine.reset();
        assert_eq!(machine.registers, [0; 16]);
        assert_eq!(machine.flags, 0);
    }

    #[test]
    fn test_mov() {
        let mut machine = Machine::new();
        let mut mem = Memory::new();

        mem.load_rom(&[
            0b0001_0001,
            0b0000_0000,
            RAM_BASE as u8,
            (RAM_BASE >> 8) as u8,
        ]); // MOV R0, RAM_BASE
        machine.step(&mut mem);
        assert_eq!(machine.registers[0], RAM_BASE);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 0b0000_0001, 0x00, 0x01]); // MOV R1, 256
        machine.step(&mut mem);
        assert_eq!(machine.registers[1], 0x0100);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0010, 0b0000_0001]); // MOV R0*, R1
        machine.step(&mut mem);
        assert_eq!(mem.read_u16(RAM_BASE), 0x0100);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0011, 0b0010_0000]); // MOV R2, R0*
        machine.step(&mut mem);
        assert_eq!(machine.registers[2], 0x0100);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0000, 0b0011_0010]); // MOV R2, R1
        machine.step(&mut mem);
        assert_eq!(machine.registers[3], 0x0100);
    }

    #[test]
    fn test_movb() {
        let mut machine = Machine::new();
        let mut mem = Memory::new();

        mem.load_rom(&[
            0b0001_0001,
            0b0000_0000,
            RAM_BASE as u8,
            (RAM_BASE >> 8) as u8,
        ]); // MOV R0, RAM_BASE
        machine.step(&mut mem);
        assert_eq!(machine.registers[0], RAM_BASE);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0101, 0b0000_0001, 0x0A]); // MOV R1, 10
        machine.step(&mut mem);
        assert_eq!(machine.registers[1], 0x0A);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0110, 0b0000_0001]); // MOV R0*, R1
        machine.step(&mut mem);
        assert_eq!(mem.read_u16(RAM_BASE), 0x0A);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0000, 0b0010_0001]); // MOV R2, R1
        machine.step(&mut mem);
        assert_eq!(machine.registers[2], 0x0A);
    }

    #[test]
    fn test_stack_mov() {
        let mut machine = Machine::new();
        let mut mem = Memory::new();

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 0, 0x0A, 0]); // MOV R0, 10
        machine.step(&mut mem);
        assert_eq!(machine.registers[0], 0x000A);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_1000, 0]); // PHR R0
        machine.step(&mut mem);
        assert_eq!(mem.read_u16(machine.registers[SP] - 2), 0x000A);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0010_0000, 1]); // PLR R1
        machine.step(&mut mem);
        assert_eq!(machine.registers[1], 0x000A);
    }

    #[test]
    fn test_add() {
        let mut machine = Machine::new();
        let mut mem = Memory::new();

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 0, 0x0A, 0]); // MOV R0, 10
        machine.step(&mut mem);
        assert_eq!(machine.registers[0], 0x000A);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 1, 0x10, 0]); // MOV R0, 16
        machine.step(&mut mem);
        assert_eq!(machine.registers[1], 0x0010);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0010_1000, 0b0001_0000]); // ADD R1, R0
        machine.step(&mut mem);
        assert_eq!(machine.registers[1], 0x001A);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0010_1001, 0, 0x0A, 0]); // ADD R0, 16
        machine.step(&mut mem);
        assert_eq!(machine.registers[0], 0x0014);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 0, 0xFF, 0xFF]); // MOV R0, 10
        machine.step(&mut mem);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 1, 0x01, 0]); // MOV R1, 10
        machine.step(&mut mem);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0010_1000, 0b0001_0000]); // ADD R1, R0
        machine.step(&mut mem);
        assert_eq!(machine.get_flag(Flag::Zero), true);
        assert_eq!(machine.get_flag(Flag::Negative), false);
        assert_eq!(machine.get_flag(Flag::Overflow), true);
        assert_eq!(machine.registers[1], 0);
    }

    #[test]
    fn test_addb() {
        let mut machine = Machine::new();
        let mut mem = Memory::new();

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0101, 0, 0x0A]); // MOVB R0, 10
        machine.step(&mut mem);
        assert_eq!(machine.registers[0], 0x0A);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0101, 1, 0x10]); // MOVB R0, 16
        machine.step(&mut mem);
        assert_eq!(machine.registers[1], 0x10);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0010_1100, 0b0001_0000]); // ADDB R1, R0
        machine.step(&mut mem);
        assert_eq!(machine.registers[1], 0x1A);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0010_1101, 0, 0x0A]); // ADDB R0, 16
        machine.step(&mut mem);
        assert_eq!(machine.registers[0], 0x14);
    }

    #[test]
    fn test_sub() {
        let mut machine = Machine::new();
        let mut mem = Memory::new();

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 0, 0x0A, 0]); // MOV R0, 10
        machine.step(&mut mem);
        assert_eq!(machine.registers[0], 0x000A);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 1, 0x0A, 0]); // MOV R1, 10
        machine.step(&mut mem);
        assert_eq!(machine.registers[1], 0x000A);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0011_0000, 0b0001_0000]); // SUB R1, R0
        machine.step(&mut mem);
        assert_eq!(machine.get_flag(Flag::Zero), true);
        assert_eq!(machine.registers[1], 0x0000);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0011_0000, 0b0001_0000]); // SUB R1, R0
        machine.step(&mut mem);
        assert_eq!(machine.get_flag(Flag::Zero), false);
        assert_eq!(machine.get_flag(Flag::Negative), true);
        assert_eq!(machine.registers[1], 0xFFF6);
    }

    #[test]
    fn test_mul() {
        let mut machine = Machine::new();
        let mut mem = Memory::new();

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 0, 0x0A, 0]); // MOV R0, 10
        machine.step(&mut mem);
        assert_eq!(machine.registers[0], 0x000A);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 1, 0x0A, 0]); // MOV R1, 10
        machine.step(&mut mem);
        assert_eq!(machine.registers[1], 0x000A);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0011_1000, 0b0001_0000]); // MUL R1, R0
        machine.step(&mut mem);
        assert_eq!(machine.registers[1], 0x0064);
    }
    #[test]
    fn test_div() {
        let mut machine = Machine::new();
        let mut mem = Memory::new();

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 0, 0x02, 0]); // MOV R0, 2
        machine.step(&mut mem);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 1, 0x0A, 0]); // MOV R1, 10
        machine.step(&mut mem);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0100_0000, 0b0001_0000]); // DIV R1, R0
        machine.step(&mut mem);
        assert_eq!(machine.registers[1], 0x0005);
    }

    #[test]
    fn test_mod() {
        let mut machine = Machine::new();
        let mut mem = Memory::new();

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 0, 0x02, 0]); // MOV R0, 10
        machine.step(&mut mem);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 1, 0x09, 0]); // MOV R1, 10
        machine.step(&mut mem);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0100_1000, 0b0001_0000]); // MOD R1, R0
        machine.step(&mut mem);
        assert_eq!(machine.registers[1], 0x0001);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 1, 0x0A, 0]); // MOV R1, 10
        machine.step(&mut mem);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0100_1000, 0b0001_0000]); // MOD R1, R0
        machine.step(&mut mem);
        assert_eq!(machine.get_flag(Flag::Zero), true);
        assert_eq!(machine.registers[1], 0x0000);
    }

    #[test]
    fn test_inc_dec() {
        let mut machine = Machine::new();
        let mut mem = Memory::new();

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0001_0001, 0, 0x02, 0]); // MOV R0, 2
        machine.step(&mut mem);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0101_0000, 0]); // INC R0
        machine.step(&mut mem);
        assert_eq!(machine.registers[0], 0x0003);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0101_1000, 0]); // DEC R0
        machine.step(&mut mem);
        assert_eq!(machine.registers[0], 0x0002);

        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0101_1000, 0]); // DEC R0
        machine.step(&mut mem);
        machine.registers[PC] = ROM_BASE;
        mem.load_rom(&[0b0101_1000, 0]); // DEC R0
        machine.step(&mut mem);
        assert_eq!(machine.get_flag(Flag::Zero), true);
        assert_eq!(machine.registers[0], 0x0000);
    }
}

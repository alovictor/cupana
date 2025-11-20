use crate::memory::{Memory, RAM_BASE, ROM_BASE, STACK_BASE};
const PC: usize = 14;
const SP: usize = 15;

pub enum Flag {
    Zero              = 0x0001,
    Carry             = 0x0002,
    Negative          = 0x0004,
    Overflow          = 0x0008,
    InterruptDisabled = 0x0010,
    InterruptPending  = 0x0020,
    Halt              = 0x0080
}

macro_rules! fetch_u8 {
    ($self:ident, $mem:ident) => {{
        let addr = $self.registers[PC];
        $self.registers[PC] = $self.registers[PC].wrapping_add(1);
        $mem.read_u8(addr)
    }};
}

macro_rules! fetch_u16 {
    ($self:ident, $mem:ident) => {{
        let addr = $self.registers[PC];
        $self.registers[PC] = $self.registers[PC].wrapping_add(2);
        $mem.read_u16(addr)
    }};
}

fn extract_registers_from_byte(byte: u8) -> (u8, u8) {
    let reg_a = (byte >> 4) & 0b1111;
    let reg_b = byte & 0b1111;
    (reg_a, reg_b)
}

pub struct Machine {
    registers: [u16; 16],
    flags: u16
}

impl Machine {
    pub fn new() -> Self {
        let mut registers = [0; 16];
        registers[PC] = ROM_BASE;
        registers[SP] = STACK_BASE;

        Machine {
            registers,
            flags: 0
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

    fn update_zn_flags(&mut self, result: u16) {
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::Negative, (result & 0x8000) != 0);
    }

    pub fn step(&mut self, mem: &mut Memory) {
        let byte = fetch_u8!(self, mem);
        let opcode = byte >> 3;
        let b = (byte >> 2) & 1;
        let mode = byte & 0b11;

        match opcode {
            // NOP
            0x00 => self.registers[PC] += 1,
            // HLT
            0x01 => self.set_flag(Flag::Halt, true),
            // MOV
            0x02 => {
                match b {
                    0 => {
                        match mode {
                            0 => {
                                let (dest, orig) = extract_registers_from_byte(fetch_u8!(self, mem));
                                println!("MOV R{}, R{}", dest, orig);
                                let value = self.registers[orig as usize];
                                self.registers[dest as usize] = value;
                            },
                            1 => {
                                let reg = fetch_u8!(self, mem);
                                let value = fetch_u16!(self, mem);
                                println!("MOV R{}, {} ; {:04X}", reg, value, value);
                                self.registers[reg as usize] = value;
                            },
                            2 => {
                                let (desti, orig) = extract_registers_from_byte(fetch_u8!(self, mem));
                                println!("MOV R{}*, R{}", desti, orig);
                                let addr = self.registers[desti as usize];
                                let value = self.registers[orig as usize];
                                mem.write_u16(addr, value);
                            },
                            3 => {
                                let (dest, orig) = extract_registers_from_byte(fetch_u8!(self, mem));
                                println!("MOV R{}, R{}*", dest, orig);
                                let addr = self.registers[orig as usize];
                                let value = mem.read_u16(addr);
                                self.registers[dest as usize] = value;
                            },
                            _ => unreachable!()
                        }
                    },
                    1 => {
                        match mode {
                            0 => {
                                let (dest, orig) = extract_registers_from_byte(fetch_u8!(self, mem));
                                println!("MOVB R{}, R{}", dest, orig);
                                let value = self.registers[orig as usize] as u8;
                                self.registers[dest as usize] = value as u16;
                            },
                            1 => {
                                let reg = fetch_u8!(self, mem);
                                let value = fetch_u8!(self, mem);
                                println!("MOVB R{}, {} ; {:04X}", reg, value, value);
                                self.registers[reg as usize] = value as u16;
                            },
                            2 => {
                                let (desti, orig) = extract_registers_from_byte(fetch_u8!(self, mem));
                                println!("MOVB R{}*, R{}", desti, orig);
                                let addr = self.registers[desti as usize];
                                let value = self.registers[orig as usize] as u8;
                                mem.write_u8(addr, value);
                            },
                            3 => {
                                let (dest, orig) = extract_registers_from_byte(fetch_u8!(self, mem));
                                println!("MOVB R{}, R{}*", dest, orig);
                                let addr = self.registers[orig as usize];
                                let value = mem.read_u8(addr);
                                self.registers[dest as usize] = value as u16;
                            },
                            _ => unreachable!()
                        }
                    },
                    _ => unreachable!()
                }
            }
            _ => {
                panic!("Unimplemented opcode: {:05b}", opcode);
            }
        }
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
    fn test_flags() {
        let mut machine = Machine::new();

        machine.set_flag(Flag::Zero, true);
        assert_eq!(machine.get_flag(Flag::Zero), true);
        assert_eq!(machine.flags, 0b0000_0001);
        
        machine.set_flag(Flag::Carry, true);
        assert_eq!(machine.get_flag(Flag::Carry), true);
        assert_eq!(machine.flags, 0b0000_0011);

        machine.set_flag(Flag::Negative, true);
        assert_eq!(machine.get_flag(Flag::Negative), true);
        assert_eq!(machine.flags, 0b0000_0111);

        machine.set_flag(Flag::Carry, false);
        assert_eq!(machine.get_flag(Flag::Carry), false);
        assert_eq!(machine.flags, 0b0000_0101);

        machine.set_flag(Flag::Overflow, true);
        assert_eq!(machine.get_flag(Flag::Overflow), true);
        assert_eq!(machine.flags, 0b0000_1101);

        machine.set_flag(Flag::InterruptDisabled, true);
        assert_eq!(machine.get_flag(Flag::InterruptDisabled), true);
        assert_eq!(machine.flags, 0b0001_1101);

        machine.set_flag(Flag::InterruptPending, true);
        assert_eq!(machine.get_flag(Flag::InterruptPending), true);
        assert_eq!(machine.flags, 0b0011_1101);

        machine.set_flag(Flag::Halt, true);
        assert_eq!(machine.get_flag(Flag::Halt), true);
        assert_eq!(machine.flags, 0b1011_1101);
    }

    #[test]
    fn test_mov() {
        let mut machine = Machine::new();
        let mut mem = Memory::new();

        mem.load_rom(&[0b0001_0001, 0b0000_0000,  RAM_BASE as u8, (RAM_BASE >> 8) as u8]); // MOV R0, 10
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
}
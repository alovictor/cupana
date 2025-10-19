use crate::memory::{Memory, ROM_BASE, RAM_BASE, RAM_END};
const PC: usize = 14;
const SP: usize = 15;

pub enum Flag {
    Zero              = 0b0000_0001,
    Carry             = 0b0000_0010,
    Negative          = 0b0000_0100,
    InterruptDisabled = 0b0000_1000,
    InterruptPending  = 0b0001_0000,
    Halt              = 0b1000_0000
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

pub struct Machine {
    registers: [u16; 16],
    flags: u8
}

impl Machine {
    pub fn new() -> Self {
        let mut registers = [0; 16];
        registers[PC] = ROM_BASE;
        registers[SP] = RAM_END;

        Machine {
            registers,
            flags: 0
        }
    }

    pub fn reset(&mut self) {
        self.registers = [0; 16];
        self.flags = 0;
    }

    fn get_flag(&self, flag: Flag) -> bool {
        (self.flags & flag as u8) != 0
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.flags |= flag as u8;
        } else {
            self.flags &= !(flag as u8);
        }
    }

    fn update_flags(&mut self, result: u16) {
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::Negative, (result & 0x8000) != 0);
        // self.set_flag(Flag::Carry, (result & 0x10000) != 0);
    }

    pub fn step(&mut self, mem: &mut Memory) {
        let byte = fetch_u8!(self, mem);
        let opcode = byte >> 3;
        let b = (byte >> 2) & 1;
        let l = (byte >> 1) & 1;
        let i = byte & 1;
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
}
use std::fmt::Display;

use crate::error::MemoryError;

use super::{Memory, STACK_SIZE};

pub struct Stack {
    data: [u8; STACK_SIZE as usize]
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: [0; STACK_SIZE as usize]
        }
    }
}

impl Memory for Stack {
    fn read_u8(&self, addr: u16) -> Result<u8, MemoryError> {
        Ok(self.data[addr as usize])
    }

    fn write_u8(&mut self, addr: u16, val: u8) -> Result<(), MemoryError> {
        self.data[addr as usize] = val;
        Ok(())
    }

    fn read_u16(&self, addr: u16) -> Result<u16, MemoryError> {
        let lo = self.data[addr as usize] as u16;
        let hi = self.data[(addr + 1) as usize] as u16;
        Ok((hi << 8) | lo)
    }

    fn write_u16(&mut self, addr: u16, val: u16) -> Result<(), MemoryError> {
        let lo = (val & 0xff) as u8;
        let hi = ((val >> 8) & 0xff) as u8;
        self.data[addr as usize] = lo;
        self.data[(addr + 1) as usize] = hi;
        Ok(())
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, byte) in self.data.iter().enumerate() {
            if i > 0x03FF {
                continue;
            }
            if i % 32 == 0 {
                write!(f, "0x{:04x} | ", i)?;
            }
            write!(f, "{:02x} ", byte)?;
            if (i + 1) % 32 == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }   
}
use std::fmt::Display;
use crate::error::MemoryError;
use super::{Memory, ROM_SIZE};

#[derive(Debug)]
pub struct Rom {
    data: [u8; ROM_SIZE as usize],
}

impl Rom {
    pub fn new() -> Self {
        Self { data: [0; ROM_SIZE as usize] }
    }

    pub fn load(&mut self, data: &[u8]) {
        self.data[..data.len()].copy_from_slice(data);
    }
}

impl Memory for Rom {
    fn read_u8(&self, addr: u16) -> Result<u8, MemoryError> {
        Ok(self.data[addr as usize])
    }

    fn write_u8(&mut self, addr: u16, _: u8) -> Result<(), MemoryError> {
        Err(MemoryError::WriteNotPermitted(addr))
    }

    fn read_u16(&self, addr: u16) -> Result<u16, MemoryError> {
        let lo = self.data[addr as usize] as u16;
        let hi = self.data[(addr + 1) as usize] as u16;
        Ok((hi << 8) | lo)
    }

    fn write_u16(&mut self, addr: u16, _: u16) -> Result<(), MemoryError> {
        Err(MemoryError::WriteNotPermitted(addr))
    }
}

impl Display for Rom {
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

#[cfg(test)]
mod tests {
    use super::*;

    // testar se o load altera o tamanho de self.data
    #[test]
    fn test_load_rom() {
        let mut rom = Rom::new();
        let data = [0, 1, 2, 3];
        rom.load(&data);
        assert_eq!(rom.data.len(), ROM_SIZE as usize);
    }
}
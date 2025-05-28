pub mod console;
pub mod serial;

// src/devices.rs
use crate::error::MemoryError; // Certifique-se que MemoryError é público e acessível
use std::fmt::Debug;

pub trait Device: Debug {
    /// Lê um byte do dispositivo em um offset relativo à sua base.
    fn read_u8(&mut self, addr_offset: u16) -> Result<u8, MemoryError>;

    /// Escreve um byte no dispositivo em um offset relativo à sua base.
    fn write_u8(&mut self, addr_offset: u16, val: u8) -> Result<(), MemoryError>;

    /// Lê uma palavra (u16, little-endian) do dispositivo.
    fn read_u16(&mut self, addr_offset: u16) -> Result<u16, MemoryError> {
        let lo = self.read_u8(addr_offset)? as u16;
        let hi = self.read_u8(addr_offset + 1)? as u16;
        Ok((hi << 8) | lo)
    }

    /// Escreve uma palavra (u16, little-endian) no dispositivo.
    fn write_u16(&mut self, addr_offset: u16, val: u16) -> Result<(), MemoryError> {
        self.write_u8(addr_offset, (val & 0xFF) as u8)?;
        self.write_u8(addr_offset + 1, ((val >> 8) & 0xFF) as u8)?;
        Ok(())
    }

    /// Retorna o intervalo de endereços globais (início, fim) que este dispositivo ocupa.
    fn aabb(&self) -> (u16, u16); // (global_start_addr, global_end_addr)

    fn check_interrupt(&mut self) -> bool;
}
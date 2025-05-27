use std::collections::VecDeque; // Para um buffer de saída, se necessário
use crate::error::MemoryError;
use crate::memory::{MMIO_BASE};
use super::Device;

pub const TERM_OUTPUT_ADDR: u16 = MMIO_BASE + 0x00; // Ex: 0xF000 para dados
pub const TERM_STATUS_ADDR: u16 = MMIO_BASE + 0x01; // Ex: 0xF001 para status
const TERM_DEVICE_SIZE: u16 = 2; // Ocupa 2 bytes (0xF000, 0xF001)


// 0x00 -> data (read, write)

#[derive(Debug)]
pub struct CupanaConsole {
    base_address: u16, // Endereço global base do dispositivo
    output_buffer: VecDeque<u8>, // Se quiser bufferizar no host
    input_buffer: VecDeque<u8>, // Se quiser bufferizar no host
}

impl CupanaConsole {
    pub fn new(base_address: u16) -> Self {
        Self {
            base_address,
            output_buffer: VecDeque::new(),
            input_buffer: VecDeque::new(),
        }
    }
}

impl Device for CupanaConsole {
    fn aabb(&self) -> (u16, u16) {
        (self.base_address, self.base_address + TERM_DEVICE_SIZE - 1)
    }

    fn read_u8(&mut self, addr_offset: u16) -> Result<u8, MemoryError> {
        match addr_offset {
            0x00 | 0x01 => {
                Ok(self.input_buffer.pop_front().unwrap_or(0))
            },
            _ => Err(MemoryError::InvalidRamAddress(self.base_address + addr_offset)),
        }
    }

    fn write_u8(&mut self, addr_offset: u16, val: u8) -> Result<(), MemoryError> {
        match addr_offset {
            0x00 | 0x01 => {
                self.output_buffer.push_back(val); // Se estiver usando buffer
                print!("{}", val as char); // Imprime o caractere no console do host
                Ok(())
            }
            _ => Err(MemoryError::InvalidRamAddress(self.base_address + addr_offset)),
        }
    }
}
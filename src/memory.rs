use std::{fmt, ops::Range};

pub const ROM_SIZE: usize = 0x8000;
pub const RAM_SIZE: usize = 0x6000;
pub const STACK_SIZE: usize = 0x1000;
pub const DEVICE_SIZE: usize = 0x1000;
pub const MEMORY_SIZE: usize = ROM_SIZE + RAM_SIZE + STACK_SIZE + DEVICE_SIZE;

pub const ROM_BASE: u16 = 0x0000;
pub const RAM_BASE: u16 = ROM_BASE + ROM_SIZE as u16;
pub const STACK_BASE: u16 = RAM_BASE + RAM_SIZE as u16;
pub const DEVICE_BASE: u16 = STACK_BASE + STACK_SIZE as u16;

pub const ROM_END: u16 = ROM_BASE + ROM_SIZE as u16 - 1;
pub const RAM_END: u16 = RAM_BASE + RAM_SIZE as u16 - 1;
pub const STACK_END: u16 = STACK_BASE + STACK_SIZE as u16 - 1;
pub const DEVICE_END: u16 = 0xFFFF;

pub struct Memory {
    rom: [u8; ROM_SIZE],
    ram: [u8; RAM_SIZE],
    stack: [u8; STACK_SIZE],
    device: [u8; DEVICE_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            rom: [0; ROM_SIZE],
            ram: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
            device: [0; DEVICE_SIZE],
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.rom[..rom.len()].copy_from_slice(rom);
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            ROM_BASE..=ROM_END => self.rom[(address - ROM_BASE) as usize],
            RAM_BASE..=RAM_END => self.ram[(address - RAM_BASE) as usize],
            STACK_BASE..=STACK_END => self.stack[(address - STACK_BASE) as usize],
            DEVICE_BASE..=DEVICE_END => self.device[(address - DEVICE_BASE) as usize],
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            ROM_BASE..=ROM_END => panic!("Cannot write to ROM address: {}", address),
            RAM_BASE..=RAM_END => self.ram[(address - RAM_BASE) as usize] = value,
            STACK_BASE..=STACK_END => self.stack[(address - STACK_BASE) as usize] = value,
            DEVICE_BASE..=DEVICE_END => self.device[(address - DEVICE_BASE) as usize] = value,
        }
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let low = self.read_u8(address) as u16;
        let high = self.read_u8(address + 1) as u16;
        (high << 8) | low
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        self.write_u8(address, value as u8);
        self.write_u8(address + 1, (value >> 8) as u8);
    }
    fn print_memory(&self, range: Range<u16>) {
        let cols = 8;
        for idx in range.step_by(cols) {
            print!("  {:04X}: ", idx);
            for i in 0..cols as u16 {
                let value = self.read_u8(idx + i);
                print!("{:02X} ", value);
            }
            println!();
        }
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Stack:")?;
        self.print_memory(STACK_BASE..STACK_BASE + 32);
        writeln!(f, "ROM:")?;
        self.print_memory(ROM_BASE..ROM_BASE + 32);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_memory_is_zeroed() {
        let mem = Memory::new();
        assert!(mem.rom.iter().all(|&byte| byte == 0));
        assert!(mem.ram.iter().all(|&byte| byte == 0));
        assert!(mem.device.iter().all(|&byte| byte == 0));
    }

    #[test]
    fn test_load_rom() {
        let mut mem = Memory::new();
        let rom_data = (0..ROM_SIZE).map(|i| (i % 256) as u8).collect::<Vec<u8>>();
        mem.load_rom(&rom_data);
        assert_eq!(&mem.rom[..], &rom_data[..]);
    }

    #[test]
    fn test_read_write_ram() {
        let mut mem = Memory::new();
        mem.write_u8(RAM_BASE, 0xAB);
        mem.write_u8(RAM_END, 0xCD);
        assert_eq!(mem.read_u8(RAM_BASE), 0xAB);
        assert_eq!(mem.read_u8(RAM_END), 0xCD);
    }

    #[test]
    fn test_read_write_stack() {
        let mut mem = Memory::new();
        mem.write_u8(STACK_BASE, 0x56);
        mem.write_u8(STACK_END, 0x78);
        assert_eq!(mem.read_u8(STACK_BASE), 0x56);
        assert_eq!(mem.read_u8(STACK_END), 0x78);
    }

    #[test]
    fn test_read_write_device() {
        let mut mem = Memory::new();
        mem.write_u8(DEVICE_BASE, 0x12);
        mem.write_u8(DEVICE_END, 0x34);
        assert_eq!(mem.read_u8(DEVICE_BASE), 0x12);
        assert_eq!(mem.read_u8(DEVICE_END), 0x34);
    }

    #[test]
    fn test_read_rom() {
        let mut mem = Memory::new();
        mem.rom[0] = 0xFE;
        mem.rom[ROM_SIZE - 1] = 0xED;
        assert_eq!(mem.read_u8(ROM_BASE), 0xFE);
        assert_eq!(mem.read_u8(ROM_END), 0xED);
    }

    #[test]
    #[should_panic(expected = "Cannot write to ROM address")]
    fn test_write_to_rom_panics() {
        let mut mem = Memory::new();
        mem.write_u8(ROM_BASE, 0xFF);
    }

    #[test]
    fn test_read_u16_little_endian() {
        let mut mem = Memory::new();
        // 0xCDBA
        mem.write_u8(RAM_BASE, 0xBA);
        mem.write_u8(RAM_BASE + 1, 0xCD);
        assert_eq!(mem.read_u16(RAM_BASE), 0xCDBA);
    }
}

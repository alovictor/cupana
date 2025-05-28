mod ram;
mod rom;
mod stack;

use crate::devices::Device;
use crate::error::MemoryError;
use ram::Ram;
use rom::Rom;
use stack::Stack;
use std::cell::RefCell;
use std::fmt;

pub const ROM_BASE: u16 = 0x0000;
pub const ROM_SIZE: u16 = 0x8000; // 32KB
pub const ROM_END: u16 = ROM_BASE + ROM_SIZE - 1;

pub const RAM_BASE: u16 = 0x8000;
pub const RAM_SIZE: u16 = 0x6000; // 28KB
pub const RAM_END: u16 = RAM_BASE + RAM_SIZE - 1;

pub const STACK_BASE: u16 = 0xE000;
pub const STACK_SIZE: u16 = 0x1000; // 4KB
pub const STACK_END: u16 = STACK_BASE + STACK_SIZE - 1;

pub const MMIO_BASE: u16 = 0xF000;
pub const MMIO_END: u16 = 0xFFFF;

pub struct MemoryBus {
    rom: Rom,
    ram: Ram,
    stack: Stack,
    devices: Vec<RefCell<Box<dyn Device>>>,
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            rom: Rom::new(),
            ram: Ram::new(),
            stack: Stack::new(),
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) {
        // TODO: Verificar sobreposição de endereços com ROM, RAM ou outros dispositivos.
        self.devices.push(RefCell::new(device));
    }

    pub fn load_rom_data(&mut self, data: &[u8]) {
        self.rom.load(data);
    }

    fn find_device(&self, addr: u16) -> Option<&RefCell<Box<dyn Device>>> {
        for dev_cell in &self.devices {
            // Precisamos de um borrow temporário para chamar aabb()
            let device_borrow = dev_cell.borrow();
            let (start, end) = device_borrow.aabb();
            if addr >= start && addr <= end {
                return Some(dev_cell);
            }
        }
        None
    }

    pub fn iter_devices(&self) -> impl Iterator<Item = &RefCell<Box<dyn Device>>> {
        self.devices.iter()
    }
}

impl Memory for MemoryBus {
    fn read_u8(&self, addr: u16) -> Result<u8, MemoryError> {
        if addr >= ROM_BASE && addr <= ROM_END {
            self.rom.read_u8(addr)
        } else if addr >= RAM_BASE && addr <= RAM_END {
            self.ram.read_u8(addr - RAM_BASE) // Ajusta para o offset da RAM
        } else if addr >= STACK_BASE && addr <= STACK_END {
             self.stack.read_u8(addr - STACK_BASE) // Ajusta para o offset da Stack
        } else if let Some(device_cell) = self.find_device(addr) {
            let mut device = device_cell.borrow_mut();
            let (dev_start, _) = device.aabb();
            device.read_u8(addr - dev_start) // Passa o offset relativo ao dispositivo
        } else {
            println!("Bus read u8: Address 0x{:04X} not mapped", addr);
            Err(MemoryError::InvalidRamAddress(addr)) // Ou um novo `AddressNotMapped`
        }
    }

    fn read_u16(&self, addr: u16) -> Result<u16, MemoryError> {
        // Verifica se o endereço e addr+1 estão dentro da mesma região
        if addr >= ROM_BASE && (addr.saturating_add(1)) <= ROM_END {
            self.rom.read_u16(addr)
        } else if addr >= RAM_BASE && (addr.saturating_add(1)) <= RAM_END {
            self.ram.read_u16(addr - RAM_BASE)
        } else if addr >= STACK_BASE && addr <= STACK_END {
             self.stack.read_u16(addr - STACK_BASE) // Ajusta para o offset da Stack
        } else if let Some(device_cell) = self.find_device(addr) {
            let mut device = device_cell.borrow_mut();
            let (dev_start, dev_end) = device.aabb();
            if addr.saturating_add(1) > dev_end {
                return Err(MemoryError::InvalidRamAddress(addr));
            }
            device.read_u16(addr - dev_start)
        } else {
            println!("Bus read u16: Address 0x{:04X} not mapped", addr);
            Err(MemoryError::InvalidRamAddress(addr))
        }
    }

    fn write_u8(&mut self, addr: u16, val: u8) -> Result<(), MemoryError> {
        if addr >= ROM_BASE && addr <= ROM_END {
            Err(MemoryError::WriteNotPermitted(addr)) // Não se pode escrever na ROM
        } else if addr >= RAM_BASE && addr <= RAM_END {
            self.ram.write_u8(addr - RAM_BASE, val)
        } else if addr >= STACK_BASE && addr <= STACK_END {
             self.stack.write_u8(addr - STACK_BASE, val) // Ajusta para o offset da Stack
        } else if let Some(device_cell) = self.find_device(addr) {
            let mut device = device_cell.borrow_mut();
            let (dev_start, _) = device.aabb();
            device.write_u8(addr - dev_start, val)
        } else {
            println!("Bus write u8: Address 0x{:04X} not mapped", addr);
            Err(MemoryError::WriteNotPermitted(addr)) // Ou InvalidAddress
        }
    }

    fn write_u16(&mut self, addr: u16, val: u16) -> Result<(), MemoryError> {
        if addr >= ROM_BASE && (addr.saturating_add(1)) <= ROM_END {
            Err(MemoryError::WriteNotPermitted(addr))
        } else if addr >= RAM_BASE && (addr.saturating_add(1)) <= RAM_END {
            self.ram.write_u16(addr - RAM_BASE, val)
        } else if addr >= STACK_BASE && addr <= STACK_END {
             self.stack.write_u16(addr - STACK_BASE, val) // Ajusta para o offset da Stack
        } else if let Some(device_cell) = self.find_device(addr) {
            let mut device = device_cell.borrow_mut();
            let (dev_start, dev_end) = device.aabb();
            device.write_u8(addr - dev_start, val as u8)
        } else {
            Err(MemoryError::WriteNotPermitted(addr))
        }
    }
}

// Para debugging, se necessário:
impl fmt::Debug for MemoryBus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemoryBus")
            .field("rom_size", &ROM_SIZE)
            .field("ram_size", &RAM_SIZE)
            .field("num_devices", &self.devices.len())
            .finish()
    }
}

pub trait Memory {
    fn read_u8(&self, addr: u16) -> Result<u8, MemoryError>;
    fn read_u16(&self, addr: u16) -> Result<u16, MemoryError>;
    fn write_u8(&mut self, addr: u16, val: u8) -> Result<(), MemoryError>;
    fn write_u16(&mut self, addr: u16, val: u16) -> Result<(), MemoryError>;
}

use std::fmt::Display;

use crate::devices::Device;
use crate::error::CError;
use crate::machine::CupanaMachine;
use crate::memory::MemoryBus;

pub struct Cupana {
    cpu: CupanaMachine,
    mem_bus: MemoryBus,
    running: bool,
}

impl Cupana {
    pub fn new() -> Self {
        Self {
            cpu: CupanaMachine::new(),
            mem_bus: MemoryBus::new(),
            running: false,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.mem_bus.load_rom_data(program);
    }

    /// Registra um novo dispositivo MMIO.
    pub fn register_device(&mut self, device: Box<dyn Device>) {
        self.mem_bus.add_device(device);
    }

    pub fn run(&mut self) -> Result<(), CError> {
        self.running = true;
        self.cpu.reset(&mut self.mem_bus)?;
        while self.running {
            self.cpu.step(&mut self.mem_bus).map_err(CError::VM)?;
            if self.cpu.has_halted() {
                self.running = false;
            }
            for device_cell in self.mem_bus.iter_devices() {
                let mut device = device_cell.borrow_mut(); // Get mutable access to the device
                if device.check_interrupt() {
                    self.cpu.request_interrupt();
                }
            }
        }
        Ok(())
    }
}

impl Display for Cupana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Cupana Machine State:")?;
        writeln!(f, "{}", self.cpu)?;
        // writeln!(f, "ROM:\n{}", self.rom)?;
        // writeln!(f, "RAM:\n{}", self.ram)?;
        writeln!(f, "Running: {}", self.running)
    }
}
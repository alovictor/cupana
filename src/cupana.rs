use crate::error::CError;
use crate::machine::CupanaMachine;
use crate::memory::{Rom, Ram};

pub struct Cupana {
    cpu: CupanaMachine,
    rom: Rom,
    ram: Ram,
    running: bool,
}

impl Cupana {
    pub fn new() -> Self {
        Self {
            cpu: CupanaMachine::new(),
            rom: Rom::new(),
            ram: Ram::new(),
            running: false,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.rom.load(program);
    }

    pub fn run(&mut self) -> Result<(), CError> {
        self.running = true;
        while self.running {
            self.cpu.step(&self.rom, &mut self.ram)?;
            if self.cpu.has_halted() {
                self.running = false;
            }
        }
        Ok(())
    }   
}
pub mod memory;
pub mod machine;

fn main() {
    let mut mem = memory::Memory::new();
    let mut machine = machine::Machine::new();

    mem.load_rom(&[0b0001_0000, 0b0001_1000, 0b0000_1000]);

    while !machine.halted() {
        machine.step(&mut mem);
    }
}

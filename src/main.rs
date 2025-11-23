pub mod assembler;
pub mod machine;
pub mod memory;

use std::io;

use crate::assembler::CupanaAssembler;

fn main() {
    let mut asm = CupanaAssembler::new("examples/simple_add.casm");
    let mut mem = memory::Memory::new();
    let code = asm.assemble();
    println!("{:?}", code);
    mem.load_rom(code.as_slice());

    let mut machine = machine::Machine::new(&mem);

    while !machine.halted() {
        machine.step(&mut mem);
    }
}

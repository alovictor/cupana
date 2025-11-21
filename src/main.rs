pub mod assembler;
pub mod machine;
pub mod memory;

use std::io;

use crate::assembler::CupanaAssembler;

fn main() {
    let mut asm = CupanaAssembler::new("examples/simple_add.casm");
    asm.assemble();
    let mut mem = memory::Memory::new();

    let mut machine = machine::Machine::new(&mem);
}

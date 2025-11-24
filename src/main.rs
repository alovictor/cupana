pub mod assembler;
pub mod machine;
pub mod memory;

use std::io::{self, Read};

use crate::{assembler::CupanaAssembler, memory::ROM_SIZE};

fn main() {
    let mut asm = CupanaAssembler::new("examples/simple_add.casm");
    let mut mem = memory::Memory::new();
    let code: &[u8] = asm.assemble();
    mem.load_rom(code);

    let mut machine = machine::Machine::new(&mem);

    loop {
        // let mut buf = String::new();
        // match io::stdin().read_line(&mut buf) {
        //     Ok(_) => match buf.pop() {
        //         Some(c) => {
        //             if c == '\n' {
        if machine.halted() {
            break;
        }
        machine.step(&mut mem);
        //                 }
        //             }
        //             None => {}
        //         },
        //         Err(e) => println!("{:?}", e),
        //     }
    }
}

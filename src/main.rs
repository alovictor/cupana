pub mod machine;
pub mod memory;
use std::io;

fn main() {
    let mut mem = memory::Memory::new();
    let mut machine = machine::Machine::new();
    let rom = [
        0b0001_0001,
        0,
        0x00,
        0x80, // mov r0, -1
        0b0001_1000,
        0, // phr r0
        0b0010_0000,
        1,           // plr r1
        0b0000_1000, // hlt
    ];

    mem.load_rom(&rom);

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler entrada");

        if input == "\n".to_string() {
            if !machine.halted() {
                machine.step(&mut mem);
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

pub mod machine;
pub mod memory;
pub mod error;
pub mod cupana;
pub mod casm;
pub mod devices;

use clap::Parser;
use casm::Assembler;
use cupana::Cupana;
use error::CError;
use devices::console::CupanaConsole; // Importe seu dispositivo
use memory::MMIO_BASE; // Importe o endereço base de MMIO

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_file: String,
    output_file: Option<String>, // Não usado no código fornecido
}

fn main() -> Result<(), CError> {
    let args = Args::parse();
    let mut assembler = Assembler::new();
    match args.output_file {
        Some(output_path) => {
            assembler.assemble_to_file(args.input_file, output_path)?
        }
        None => {
            let program_bytes = assembler.assemble_file(args.input_file)?;
            let mut cupana_vm = Cupana::new();
            cupana_vm.load_program(program_bytes);
        
            let terminal = CupanaConsole::new(MMIO_BASE); 
            cupana_vm.register_device(Box::new(terminal));
        
            cupana_vm.run()?;
        }
    }
    
    
    // println!("{}", cupana_vm); // Para ver o estado final
    Ok(())
}
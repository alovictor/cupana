pub mod machine;
pub mod memory;
pub mod error;
pub mod cupana;
pub mod casm;

use clap::Parser;
use casm::Assembler;
use cupana::Cupana;
use error::CError;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_file: String,
    output_file: Option<String>,
}

fn main() -> Result<(), CError> {
    let args = Args::parse();
    let mut assembler = Assembler::new();
    let program = assembler.assemble_file(args.input_file)?;
    let mut cupana = Cupana::new();
    cupana.load_program(&program);
    cupana.run()?;
    println!("{}", cupana);
    Ok(())
}

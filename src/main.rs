use clap::Parser;

pub mod assembler;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input assembly file path
    input_file: String,

    /// Output binary file path
    output_file: String,
}

fn main() {
    let args = Args::parse();

    println!(
        "Assembling '{}' to '{}'",
        args.input_file, args.output_file
    );

    match assembler::driver::assemble_file(&args.input_file, &args.output_file) {
        Ok(()) => {
            println!("Assembly successful!");
        }
        Err(e) => {
            eprintln!("{}", e); 
            std::process::exit(1);
        }
    }
}

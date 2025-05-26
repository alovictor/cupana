use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_file: String,
    output_file: Option<String>,
}

fn main() {
    let args = Args::parse();
}

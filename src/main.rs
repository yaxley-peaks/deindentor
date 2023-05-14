use clap::Parser;
use deindentor::*;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let args = CLI::parse();
    let file = match fs::read_to_string(&args.input) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Input: Error: {}", e);
            return;
        }
    };
    let outfile = match File::create(&args.output) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Output: Error: {}", e);
            return;
        }
    };
    let mut writer = BufWriter::new(outfile);
    writeln!(&mut writer, "{}", generate_result(&file)).unwrap();
}

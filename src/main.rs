use std::fs::File;
use std::io::{ BufWriter, Write };

pub mod error;
use error::Error;

fn print_usage(prog_name: &str) {
    println!("Valid Usage: {prog_name} [input_path] [output_path]");
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        print_usage(&args[0]);
        return Err(Error::InvalidUsage);
    }

    let i_data: Vec<u8> = std::fs::read(args[1].as_str())?;
    let mut o_file: BufWriter<File> = BufWriter::new(
        File::options()
            .create(true)
            .write(true)
            .open(args[2].as_str())?
        );
    let var_name: String = args[1].to_uppercase()
        .replace("/", "_")
        .replace(".", "_")
        .replace("\\", "_");
    write!(o_file, "\n\npub const {}: [u8; {}] = [", var_name, i_data.len())?;
    const WIDTH: usize = 10usize;
    for i in 0..i_data.len() {
        if i % WIDTH == 0 { 
            write!(o_file, "\n    ")?;
        }
        write!(o_file, "0x{:02x}, ", i_data[i])?;
    }
    write!(o_file, "\n];\n")?;
    o_file.flush()?;

    Ok(())
}

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn run(args: impl Iterator<Item = String>) -> io::Result<()> {
    for arg in args {
        print_file(arg)?;
    }

    Ok(())
}

fn print_file(filename: String) -> io::Result<()> {
    let bytes = std::fs::read(filename).unwrap();
    
    for bytes in bytes.chunks(50) {
        print!("{}", String::from_utf8_lossy(bytes));

        // let char = byte as char;
        // print!("{}", char);
    }

    Ok(())
}

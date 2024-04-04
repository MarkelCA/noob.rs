use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn run(args: impl Iterator<Item = String>) -> io::Result<()> {
    for arg in args {
        print_file(arg)?;
    }

    Ok(())
}

fn print_file(filename: String) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

use std::io::{BufReader, Read, Write};
use std::fs::File;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    files: Vec<String>
}

const BUFFER_CAPACITY: usize = 1024 * 64;

fn main()-> std::io::Result<()> {
    let args = Args::parse();

    for f in args.files {
        print_file(f)?
    }

    Ok(())
}

fn print_file(filename: String) -> std::io::Result<()>{
    let f = File::open(filename)?;

    let mut reader = BufReader::with_capacity(BUFFER_CAPACITY, f);
    let mut buffer = [0; BUFFER_CAPACITY ];
    let mut stdout = std::io::stdout();

    loop {
         let n = reader.read(&mut buffer).unwrap();
         if n > 0 {
             stdout.write(&buffer[0..n])?;
         } else {
             break;
         }
    }

    Ok(())
}

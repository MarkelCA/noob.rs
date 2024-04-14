use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::fs;
use std::path::PathBuf;

pub fn run(text: String, file_path: String, recursive: bool) -> std::io::Result<()> {

    if !recursive {
        grep_file(file_path, &text)?;
    } else {
        let path = PathBuf::from(file_path);
        grep_dir(path, &text);
    }

    Ok(())
}

fn grep_dir(path: PathBuf, text: &str) {
    let paths = fs::read_dir(path).unwrap();
    for p in paths {
        let p = p.unwrap().path();
        if p.is_dir() {
            grep_dir(p, text);
        } else {
            grep_file(p.display().to_string(), text).unwrap_or_else(|err| {
                eprintln!("Error reading file {} ({err})", p.display())
            });
        }
    }
}


const BUFFER_CAPACITY: usize = 1024 * 64;
fn grep_file(file_path: String, text: &str) -> std::io::Result<()> {
    let path = PathBuf::from(file_path);
    let file = File::open(&path)?;
    let reader = BufReader::with_capacity(BUFFER_CAPACITY, file);

    let mut line = String::new();
    for byte in reader.bytes() {
        let b = byte?;
        if b == b'\n' {
            if line.contains(text) {
                println!("{}",line);
                return Ok(());
            }
            line.clear();
        } else {
            line.push(char::from(b))
        }
    }

    Ok(())
}

/**
 * Alternative implementation for grep_file.
 * Only works for text files (breaks with non UTF-8 characters)
 */
fn _grep_text_file(file_path: String, text: &str) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

     for line in reader.lines() {
         let line = line?;
         if line.contains(&text) {
             println!("{}", line);
         }
     }
    Ok(())
}

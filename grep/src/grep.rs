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
            let x = grep_file(p.display().to_string(), text);
            x.unwrap()
        }
    }
}


fn grep_file(file_path: String, text: &str) -> std::io::Result<()> {
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    let bytes = reader.bytes();
    for byte in bytes {
        let byte = byte.unwrap();
        if byte == text.as_bytes()[0] {
            println!("{} -> {:?}", file_path, text);
        }
    }

    // for line in reader.lines() {
    //     println!("{} -> {:?}",file_path, line);
    //     let line = line?;
    //     if line.contains(&text) {
    //         println!("{}", line);
    //     }
    // }
    Ok(())
}

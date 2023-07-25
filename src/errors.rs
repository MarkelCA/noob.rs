use std::fs::File;

pub fn test() {
    let file_name = "Cargo.toml";
    let file_result = File::open(file_name);

    let greeting_file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file \"{}\": {:?}",file_name, error),
    };

    println!("{:?}", greeting_file);
}

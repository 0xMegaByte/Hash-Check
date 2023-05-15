use std::fs::File;
use std::io::{self, Read};

pub fn calculate_md5(file_path: &String) -> io::Result<String> {
    let mut file = File::open(file_path).expect("File not found!");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("Failed to read file!");

    let hash = md5::compute(&buffer);

    Ok(format!("{:x}", hash))
} //File object closed after this line

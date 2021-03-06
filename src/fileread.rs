use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

pub fn read_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let _ = match File::open(filename) {
		Ok(file) => return Ok(io::BufReader::new(file).lines()),
		Err(e) => return Err(e),
	};
}

pub fn file_to_str(filename: &str) -> String {
    let mut file = match read_file(filename) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e);
            process::exit(0x0010);
        },
    };
    let mut text = String::new();
    while let Some(line) = file.next() {
        text.push_str(&line.unwrap());
    }
    text
}

pub fn file_to_str_ascii_only(filename: &str) -> String {
    let file_str = file_to_str(filename);
    let mut output = String::new();
    
    for char in file_str.chars() {
        if char >= 'a' && char <= 'z' {
            output.push(char);
        } else if char >= 'A' && char <= 'Z' {
            output.push(char.to_ascii_lowercase());
        }
    }

    output
}
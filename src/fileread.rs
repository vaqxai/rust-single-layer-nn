use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
        Err(e) => panic!("{}", e),
    };
    let mut text = String::new();
    while let Some(line) = file.next() {
        text.push_str(&line.unwrap());
    }
    text
}
use std::fs;
use std::fs::File;
use std::io::Result;
use std::io::{self, BufRead};

pub fn read(path: &str) -> Result<File> {
    let path = fs::canonicalize(path).expect("File not found");
    let file = File::open(path).expect("Cannot open file");
    Ok(file)
}

pub fn read_lines(path: &str) -> Result<Vec<String>> {
    let file = read(path).expect("Issues with opening file");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();
    Ok(lines)
}

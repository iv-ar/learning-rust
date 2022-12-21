use std::{io::{BufReader, BufRead, Error}, fs::File};

pub fn find_matches(file: &File, pattern: &str) -> Result<Vec<String>, Error> {
    let reader = BufReader::new(file);
    let mut result = Vec::<String>::new();
    for line in reader.lines() {
        let line_val = line.unwrap();
        if line_val.contains(&pattern) {
            result.push(line_val);
        }
    }
    return Ok(result);
}
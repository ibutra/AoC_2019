use std::fs::File;
use std::io::{BufReader, BufRead, Read};


pub struct Input {
    filename: String
}

impl Input {
    pub fn new(filename: &str) -> Self {
        Input {
            filename: filename.to_string(),
        }
    }

    fn open(&self) -> BufReader<File> {
        let file = File::open(&self.filename).unwrap();
        BufReader::new(file)
    }

    pub fn as_strings(&self) -> Vec<String> {
        let mut strings = Vec::new();
        for line in self.open().lines() {
            strings.push(line.unwrap())
        }
        strings
    }

    pub fn as_string(&self) -> String {
        let mut buffer = String::new();
        self.open().read_to_string(&mut buffer).unwrap();
        buffer
    }


    pub fn as_i64(&self) -> Vec<i64> {
        let mut numbers = Vec::new();
        for line in self.as_strings() {
            let int = line.parse::<i64>().unwrap();
            numbers.push(int);
        }
        numbers
    }

    
}

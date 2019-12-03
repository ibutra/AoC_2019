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
        self.open().lines().map(|x| x.unwrap()).collect()
    }

    pub fn as_string(&self) -> String {
        let mut buffer = String::new();
        self.open().read_to_string(&mut buffer).unwrap();
        buffer
    }


    pub fn as_i64(&self) -> Vec<i64> {
        self.open().lines().map(|x| x.unwrap().parse::<i64>().unwrap()).collect()
    }

    
}

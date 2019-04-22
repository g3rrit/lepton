use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::path::PathBuf;

use crate::token::*;

pub struct Input {
    path: PathBuf,
    reader: BufReader<File>,
    
    from: String,
    to: String,
}

impl Input {
    pub fn new(path: &str) -> Self {
        let path = PathBuf::from(path);
        let file = match File::open(path.as_path()) {
            Err(why) => panic!("couldn't open {}: {}", path.as_path().display(), why.description()),
            Ok(file) => file,
        };
        
        Self {
            path: path,
            reader: BufReader::new(file),
            from: String::new(),
            to: String::new(),
        }
    }
    
    pub fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }
    
    pub fn next(&mut self) -> Option<char> {
        let c = match self.from.pop() {
            Some(c) => c,
            None => {
                match self.reader.read_line(&mut self.from) {
                    Ok(len) => if len <= 0 { return None; },
                    Err(why) => panic!("unable to read from file: {}", why),
                }
                self.from = self.from.chars().rev().collect();
                self.from.pop().unwrap()
            },
        };
        
        self.to.push(c);

        Some(c)
    }
    
    pub fn rewind(&mut self, n: u32) {
        for _ in 0..n {
            match self.to.pop() {
                Some(c) => self.from.push(c),
                None => {
                    comp_warn!("unable to rewind {} Chars", n);
                    return;
                }
            }
        }
    }
    
    pub fn commit(&mut self) {
        self.to.clear();
    }
}


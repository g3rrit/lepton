use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use crate::token::*;

pub struct Input {
    path: PathBuf,
    file: File,
}

impl Input {
    pub fn new(path: &str) -> Self {
        let path = PathBuf::from(path);
        let file = match File::open(path.as_path()) {
            Err(why) => panic!("couldn't open {}: {}", path.as_path().display(), why.description()),
            Ok(file) => file,
        };
        
        Self {
            path,
            file
        }
    }
    
    pub fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }
    
    pub fn next(&self) -> Token {
        Token::LCB
    }
}


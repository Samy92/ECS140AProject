#![allow(non_snake_case)]

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::stdin;


struct CStream {
	filename: String,
	content: String,
}



impl CStream {
    fn new(f: String) -> CStream {
        CStream {
            filename: f.to_string(),
            content: String::new(),
        }
    }


    fn set_content(&mut self) -> io::Result<()> {
        let file = File::open(self.filename.as_str())?;
        let mut buf_reader = io::BufReader::new(file);
        buf_reader.read_to_string(&mut self.content).unwrap();
        Ok(())
        
    }

}



fn main() {
    let mut fname = String::new();
    println!("Enter the file name:");
    stdin().read_line(&mut fname)
        .ok()
        .expect("Failed to read line");
    let len = fname.trim_end_matches(&['\r', '\n'][..]).len();
    fname.truncate(len);
    println!("{}", fname);
    let mut ex = CStream::new(fname);
    ex.set_content().unwrap();
	println!("{}", ex.content);
}

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

enum TokenType {
    IntConstant, 
    FloatConstant, 
    Keyword, 
    Operator, 
    Identifier, 
    Invalid,
}

struct Token {
    text: String,
    token_type: TokenType,
    line_num: i32,
    char_pos: i32,
    total_pos: i32,
}

impl Token {
    fn new(f: String, tt: TokenType) -> Token {
        Token {
            text: f.to_string(),
            token_type: tt,
            line_num: -1,
            char_pos: -1,
            total_pos: -1,
        }
    }

    // get the next character
    fn getNextChar(&mut self)->char{ 
        cur_pos = self.text.chars().nth(self.total_pos).unwrap();
        if cur_pos == '\n'{
            self.char_pos =  0;
            self.line_num =  self.line_num + 1;
        }
        else{
            self.char_pos =  self.char_pos + 1;
        }
        self.total_pos = self.total_pos + 1
        return self.text.chars().nth(self.total_pos).unwrap();
    }

    fn get_TokenType(mut self, text) -> TokenType {

        let keyword = vec!["unsigned", "char", "short", "int", "long", "float", "double", "while", "if", "return", "void", "main"]
        let operator = vec!["(",",",")","{","}","=","==","<",">","<=",">=","!=","+","-","*","/",";"]
        let float = vec!['.']

        // Check if text is IntConstant or FloatConstant TokenType:
        if isnumeric(text.chars().nth(0).unwrap()) || text.chars().nth(0).unwrap() == '-'  {
            if float.contains('.'){
                return TokenType::FloatConstant;
            }
            else{
                return TokenType::IntConstant;
            }
        }
        
        // Check if text is Keyword TokenType:
        else if keyword.contains(text){ return TokenType:Keyword;}

        // Check if text is Operator TokenType:
        else if operator.contains(text){ return TokenType:Operator;}
        
        // Check if text is Identifier TokenType:
        else if(text.chars().nth(0).unwrap()=='_' || isalphabetic(text.chars().nth(0).unwrap())){
            return TokenType:Identifier;
        }

        // Check if text is Invalid TokenType:
        else{
            return TokenType:Invalid;
        }
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

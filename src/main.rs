#![allow(non_snake_case)]



use std::fmt;


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

#[derive(Debug)]
enum TokenType {
    IntConstant, 
    FloatConstant, 
    Keyword, 
    Operator, 
    Identifier, 
    Invalid,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}

struct Token {
    text: String,
    token_type: TokenType,

}

impl Token {
    fn new(f: String, tt: TokenType) -> Token {
        Token {
            text: f.to_string(),
            token_type: tt,
        }
    }
    
    fn get_next_token(f: String) -> Vec<String>{
        let mut vec = Vec::new();
        let mut st = String::new();
        let mut char_pos: i32 = -1;
        let mut line_num = -1;
        for c in f.chars(){
            char_pos = char_pos + 1;
            if c != ' ' && c != '(' && c != ')' && c != ':' && c != '\n'  && c != ';'&& c != '{' && c != '}'{
                st.push(c);
            }else{
                if c == '\n'{
                    char_pos = 0;
                    line_num = line_num + 1;
                    continue;
                }
                let copy = st.clone();
                let ch = c.to_string();
                if st != ""{
                    let x: usize = char_pos as usize;
                    let mut y = copy.len();
                    println!("Token {} Char Position {} Line Position {}", copy, x - y, line_num);
                    vec.push(copy);
                    st.clear();
                }
                if ch != "" && ch != "\n" && ch != " " && ch != "\r" {
                    println!("Token {} Char Position {} Line Position {}", ch, char_pos, line_num);
                    vec.push(ch);

                }
            }
        }
        return vec;
    }

    fn get_TokenType(tok: String) -> TokenType{
        // Check if text is IntConstant
        let keyword = vec!["unsigned", "char", "short", "int", "long", "float", "double", "while", "if", "return", "void", "main"];
        let operator = vec!["(",",",")","{","}","=","==","<",">","<=",">=","!=","+","-","*","/",";"];

        // Check if text is IntConstant or FloatConstant TokenType:
        if tok.chars().nth(0).unwrap().is_numeric() || tok.chars().nth(0).unwrap() == '-'  {
            if tok.contains('.'){
                return TokenType::FloatConstant;
            }
            else{
                return TokenType::IntConstant;
            }
        }

        // Check if text is Keyword TokenType:
        else if keyword.contains(&&*tok){ return TokenType::Keyword;}

        // Check if text is Operator TokenType:
        else if operator.contains(&&*tok){ return TokenType::Operator;}

        // Check if text is Identifier TokenType:
        else if tok.chars().nth(0).unwrap()=='_' || tok.chars().nth(0).unwrap().is_alphabetic(){
            return TokenType::Identifier;
        }

        // Check if text is Invalid TokenType:
        else{
            return TokenType::Invalid;
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
    let all_tokens = Vec::from(Token::get_next_token(ex.content));
    println!("{:?}", all_tokens);
    for n in all_tokens{
        let x = n.clone();
        let tokentype = Token::get_TokenType(n);
        println!("Token {} - Token Type: {}", x, tokentype);
    }

} 
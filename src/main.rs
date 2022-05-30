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
struct Scanner {
}

impl Scanner {
    fn get_next_token(f: String) -> (Vec<String> , Vec<i32> ,Vec<i32>){
        let mut char_pos: i32 = -1;
        let mut line_num = -1;
        let mut vec = Vec::new();
        let mut char_vec = Vec::new();
        let mut line_vec = Vec::new();
        let mut st = String::new();
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
            char_vec.push(char_pos);
            line_vec.push(line_num);
        }
        return (vec, char_vec, line_vec);
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

struct parser{
    // struct elements
}

impl parser{

    // EBNF Rules

    fn tokenCheker(t1: String, t2: String) -> String{
        if t1 == 
    }

    fn Program(&mut self)->bool{
        let mut temp = 0;
        while self.Declaration()==true{
            temp = 1;
        }
        if temp==1{
            if self.MainDeclaration()==true{
                while self.FunctionDefinition()==true{
                    self.FunctionDefinition();
                }
                return true;
            }
            else{
                return false;
            }
        }
        else{
            return false;
        }
    }

    fn Declaration(&mut self)->bool{
        if self.DeclarationType()==true{
            if VariableDeclaration()==true || FunctionDeclaration()==true{
                return true;
            }
        }
        else{
            return false;
        }
    }

    fn MainDeclaration(&mut self, s: string)->bool{
        if 
    }

    fn FunctionDefinition(&mut self)->bool{
        if self.DeclarationType()==true{
            if self.ParameterBlock()==true{
                if self.Block()==true{
                    return true;
                }
            }
        }
        else{
            return false;
        }
    }

    fn DeclarationType(&mut self)->bool{
        if self.DataType()==true{
            if self.Identifier()==true{
                return true;
            }
        }
        else{
            return false;
        }
    }

    fn VariableDeclaration(&mut self)->bool{
        // TODO
    }

    fn FunctionDeclaration(&mut self)->bool{
        if self.ParameterBlock()==true{
            return true;
        }
        else{
            return false;
        }
    }

    fn Block(&mut self)->bool{
        // TODO
    }

    fn ParameterBlock(&mut self)->bool{
        // TODO
    }

    fn DataType(&mut self)->bool{
        if self.IntegerType()==true || self.FloatType()==true{
            return true;
        }
        else{
            return false;
        }
    }

    fn Constant(&mut self)->bool{
        if self.IntConstant()==true || self.FloatConstant()==true{
            return true;
        }
        else{
            return false;
        }
    }

    fn Statement(&mut self)->bool{
        if self.Assignment() == true || self.WhileLoop() == true || self.IfStatement() == true || self.ReturnStatement() == true || self.Expression() == true {
            return true;
        }
        else{
            return false;
        }
    }

    fn Parameter(&mut self)->bool{
        // TODO
    }

    fn IntegerType(&mut self)->bool{
        // TODO
    }
    
    fn FloatType(&mut self)->bool{
        // TODO
    }

    fn Assignment(&mut self)->bool{
        // TODO
    }

    fn WhileLoop(&mut self)->bool{
        // TODO
    }

    fn IfStatement(&mut self)->bool{
        // TODO
    }

    fn ReturnStatement(&mut self)->bool{
        // TODO
    }

    fn Expression(&mut self)->bool{
        if self.SimpleExpression()==true{
            if self.RelationOperator()==true {
                if self.SimpleExpression()==true{
                    return true;
                }
            }
            return true;
        }
        else{
            return false;
        }
    }

    fn SimpleExpression(&mut self)->bool{
        let mut temp = 0;
        if self.Term()==true{
            while self.AddOperator()==true & self.Term()==true{
                temp = 1
            }
            if temp==1{
                return true;
            }
        }
        else{
            return false;
        }
    }

    fn Term(&mut self)->bool{
        let mut temp = 0;
        if self.Factor()==true{
            while self.MultOperator()==true & self.Factor()==true{
                temp = 1
            }
            if temp==1{
                return true;
            }
        }
        else{
            return false;
        }
    }

    fn Factor(&mut self)->bool{
        // TODO
    }

    fn RelationOperator(&mut self)->bool{
        // TODO
    }

    fn AddOperator(&mut self)->bool{
        // TODO
    }

    fn MultOperator(&mut self)->bool{
        // TODO
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
    let mut all_tokens = Vec::new();
    let mut all_char = Vec::new();
    let mut all_lines = Vec::new();
    (all_tokens, all_char, all_lines) = Scanner::get_next_token(ex.content);

    /*println!("Tokens {:?}", all_tokens);
    println!("Char pos {:?}", all_char);
    println!("Line Num {:?}", all_lines);*/
    let mut y =  all_tokens.len() - 1;
    for i in 0..y{
        let mut res = String::from(Parser::tokenCheker( all_tokens[i],  all_tokens[i+1]));

        
    }

} 
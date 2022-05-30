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
#[derive(PartialEq)]
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

/*struct Parser {
    all_Token : Vec<String>,
    i: usize,
}

impl Parser {

    // EBNF Rules

    fn tokenCheker(all_token:Vec<String>){
            Parser {
                all_Token: all_token,
                i: 0,
            }
    }

    fn Program(&mut self)->bool{
        let mut temp = 0;
        while self.Declaration()==true{ // (true, int)
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
            if self.VariableDeclaration()==true || self.FunctionDeclaration()==true {
                return true;
            }else{
                return false;
            }
        }else{
            return false;
        }
    }

    fn MainDeclaration(&mut self)->bool{ // bool, int (true, i)
        if self.all_Token[self.i] == "void"{
            self.i = self.i+1;
            if self.all_Token[self.i] == "main"{
                self.i = self.i+1;
                if self.all_Token[self.i] == "("{
                    self.i = self.i+1;
                    if self.all_Token[self.i] == ")"{
                        if self.Block()==true{
                            return true;
                        }
                    }else{
                        return false;
                    }
                }
            }
        }
        else{
            return false;
        }
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
        if self.all_Token[self.i] == "="{
            self.i = self.i + 1;
            if self.Constant()==true{
                if self.all_Token[self.i] == ";"{
                    self.i = self.i + 1;
                    return true;
                }
            }
        }   
        else if self.all_Token[self.i] == ";"{
            self.i = self.i + 1;
            return true;
        }
        else{
            return false;
        }
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
        if self.all_Token[self.i] == "{"{
            let mut temp = 0;
            self.i = self.i + 1;
            while self.Declaration(self.i)==true{
                temp = 1;
            }
            while self.Statement(self.i)==true{
                temp = 2;
            }
            while self.FunctionDefinition(self.i)==true{
                temp = 3;
            }
            if self.all_Token[self.i] == "}"{
                self.is = self.i+1;
                temp = 4;
            }
            if temp==4{
                return true;
            }
            else{
                return false;
            }
        }
    }

    fn ParameterBlock(&mut self)->bool{
        if self.DataType()==true{
            if self.all_Token[self.i] == TokenType::Identifier{
                self.i = self.i + 1;
                return true;
            }
        }
        else{
            return false;
        }
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
        if self.DataType()==true{
            if self.Identifier()==true{
                return true;
            }
        }
        else{
            return false;
        }
    }

    fn IntegerType(&mut self)->bool{
        if self.all_Token[self.i] == "unsigned"{
            self.i = self.i + 1;
        }
        if self.all_Token[self.i] == "char" || self.all_Token[self.i] == "short"|| self.all_Token[self.i] == "int" || self.all_Token[self.i] == "long"{ 
            self.i = self.i + 1;
            return true;
        }
        return false;
    }
    
    fn FloatType(&mut self)->bool{
        if self.all_Token[self.i] == "float" || self.all_Token[self.i] == "double"{
            self.i = self.i + 1;
            return true;
        }
        return false;
    }

    fn Assignment(&mut self)->bool{
        let mut temp = 0;
        if self.Identifier()==true{
            if self.all_Token[self.i] == "="{
                self.i = self.i + 1;
                while self.Identifier()==true && self.all_Token[self.i] == "="{
                    self.i = self.i + 1;
                    temp = 1;
                }
                if temp==1{
                    if self.Expression()==true{
                        if self.all_Token[self.i] == ";"{
                            self.i = self.i + 1;
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    fn WhileLoop(&mut self)->bool{
        if self.all_Token[self.i] == "while"{
            self.i = self.i + 1;
            if self.all_Token[self.i] == "("{
                self.i = self.i + 1;
                if self.Expression()==true{
                    if self.all_Token[self.i] == ")"{
                        self.i = self.i + 1;
                        if self.Block()==true{
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    fn IfStatement(&mut self)->bool{
        if self.all_Token[self.i] == "if"{
            self.i = self.i + 1;
            if self.all_Token[self.i] == "("{
                self.i = self.i + 1;
                if self.Expression()==true{
                    if self.all_Token[self.i] == ")"{
                        self.i = self.i + 1;
                        if self.Block()==true{
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    fn ReturnStatement(&mut self)->bool{
        if self.all_Token[self.i] == "return"{
            self.i = self.i + 1;
            if self.Expression()==true{
                if self.all_Token[self.i] == ";"{
                    self.i = self.i + 1;
                    return true;
                }

            }
        }
        return false;
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
            while self.AddOperator()==true && self.Term()==true{
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
            while self.MultOperator()==true && self.Factor()==true{
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
        let mut temp1 = 0;
        let mut temp2 = 0;
        if self.all_Token[self.i] == "("{
            self.i = self.i + 1;
            if self.Expression()==true{
                if self.all_Token[self.i] == ")"{
                    self.i = self.i + 1;
                    if self.Block()==true{
                        return true;
                    }
                }
            }else{
                return false;
            }
        }

        else if self.Constant()==true{
            return true;
        }

        else if self.Identifier()==true{
            if self.all_Token[self.i] == "("{
                self.i = self.i + 1;
                if self.Expression()==true{
                    while self.all_Token[self.i] == "," && self.Expression()==true{
                        self.i = self.i + 1;
                        temp2 = 1;
                    }
                    if temp2==1{
                        if self.all_Token[self.i] == ")"{
                            self.i = self.i + 1;
                            return true;
                        }
                    }
                    
                }
            }
            return true;
        }

        else{
            return false;
        }
        
        
    }

    fn RelationOperator(&mut self)->bool{
        if self.all_Token[self.i] == "==" || self.all_Token[self.i] == "<" || self.all_Token[self.i] == ">" || self.all_Token[self.i] == "<=" || self.all_Token[self.i] == ">=" || self.all_Token[self.i] == "!="{
            return true;
        }
        else{
            return false;
        }
    }

    fn AddOperator(&mut self)->bool{
        if self.all_Token[self.i] == "+"{
            return true;
        }
        else if self.all_Token[self.i] == "-"{
            return true;
        }
        else{
            return false;
        }
    }

    fn MultOperator(&mut self)->bool{
        if self.all_Token[self.i] == "*"{
            return true;
        }
        else if self.all_Token[self.i] == "/"{
            return true;
        }
        else{
            return false;
        }
        
    }
}*/

fn x_to_XHTML(s: String, finame: String)  -> std::io::Result<()>{
    
    let mut toks: Vec<String> = Vec::new();
    let mut st = String::new();
    for c in s.chars(){
        if c != ' ' && c != '(' && c != ')' && c != ':' && c != '\n'  && c != ';'&& c != '{' && c != '}'{
            st.push(c);
        }else{
 
            let copy = st.clone();
            let ch = c.to_string();
            toks.push(copy);
            toks.push(ch);
            st.clear();
        }
    }

    let mut x = toks.len()-1;
    for i in 0..x{
        let cop3 = toks[i].clone();
        let tt = Token::get_TokenType(cop3);
        if tt == TokenType::Identifier{
            let yell = String::from("<p style=");
            let yell1 = '"';
            let yell2 = String::from("color:#FFFF00");
            let yell3 = String::from(";>");
            let yell4 = String::from("</p>");
            toks[i].insert_str(0, &yell);
            toks[i].insert(1, yell1);
            toks[i].insert_str(2, &yell2);
            toks[i].insert(3, yell1);
            toks[i].insert_str(4, &yell2);
            toks[i].push_str(&yell4);
        }if tt == TokenType::IntConstant || tt == TokenType::FloatConstant {
            let yell = String::from("<p style=");
            let yell1 = '"';
            let yell2 = String::from("color:#00FFFF");
            let yell3 = String::from(" font-weight:bold;>");
            let yell4 = String::from("</p>");
            toks[i].insert_str(0, &yell);
            toks[i].insert(1, yell1);
            toks[i].insert_str(2, &yell2);
            toks[i].insert(3, yell1);
            toks[i].insert_str(4, &yell2);
            toks[i].push_str(&yell4);
        }if tt == TokenType::Keyword || tt == TokenType::Operator {
            let yell = String::from("<p style=");
            let yell1 = '"';
            let yell2 = String::from("color:#FFFFFF");
            let yell3 = String::from(" font-weight:bold;>");
            let yell4 = String::from("</p>");
            toks[i].insert_str(0, &yell);
            toks[i].insert(1, yell1);
            toks[i].insert_str(2, &yell2);
            toks[i].insert(3, yell1);
            toks[i].insert_str(4, &yell2);
            toks[i].push_str(&yell4);
        }
    }

    let mut text = String::new();
    for i in 0..x{
        text.push_str(&toks[i]);
    }
    let tx = format!("<!DOCTYPE html> <html> <head> <title>My Page</title> </head> <body> {} </body>/html>", text);
    let mut xhtml = String::from(tx);
    let title = format!("{}.xhtml", finame);
    let mut file = File::create(title)?;
    file.write_all(xhtml.as_bytes())?;
    Ok(())
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
    let cop2 = fname.clone();
    let mut ex = CStream::new(fname);
    ex.set_content().unwrap();
    println!("{}", ex.content);
    let mut all_tokens = Vec::new();
    let mut all_char = Vec::new();
    let mut all_lines = Vec::new();
    let cop1 = ex.content.clone();
    (all_tokens, all_char, all_lines) = Scanner::get_next_token(ex.content);
    for v in all_tokens{
        let cop3 = v.clone();
        let m = Token::get_TokenType(v);
        println!("Token {} Token Type {}", cop3, m);
    }
    /*println!("Tokens {:?}", all_tokens);
    println!("Char pos {:?}", all_char);
    println!("Line Num {:?}", all_lines);*/
    //let p = Parser::tokenCheker(all_tokens);
    //let boolResult = p::Program();
    //let proj = x_to_XHTML(cop1, cop2);

} 


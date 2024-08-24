pub const SINGLE_LEXEMS: &[char] = &[',',':', '(', ')'];

pub const OP_LEXEMS: &[&'static str] = &["+", "-", "/", "*", "&", "|", "^","<<", ">>"];

#[derive(Debug, Clone, PartialEq)]
pub enum LexemType{
    Ident,
    Single,
    Number{
        radix: usize
    },
    String,
    Operator,
    Closure {
        args: [Box<Lexem>; 3]
    },
    NewLine
}

impl PartialEq for Box<Lexem>{
    fn eq(&self, other: &Self) -> bool {
        self.ttype.eq(&other.ttype)
    }
}

impl std::fmt::Display for LexemType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match self{
            LexemType::Single => {write!(f, "Single")},
            LexemType::Number {..} => {write!(f, "Number")},
            LexemType::Ident => {write!(f, "Indent")},
            LexemType::NewLine => {write!(f, "NewLine")},
            LexemType::String => {write!(f, "String")}
            LexemType::Operator => {write!(f, "Operator")}
            LexemType::Closure {..} => {write!(f, "Closure")}
        }
    }
}
#[derive(Debug, Clone)]
pub struct Lexem{
    pub value: String,
    pub ttype: LexemType,
    pub filename: String,
    pub row: usize,
    pub col: usize
}


impl Lexem{
    pub fn new(value: String, ttype: LexemType, row: usize, col: usize, filename: String) -> Lexem{
        Lexem { value, ttype, row, col, filename}
    }
}

pub struct Lexer{
    content: String,
    source_filename: String,
    cursor: usize,
    row: usize,
    col: usize,
    pub lexems: Vec<Lexem>,
}

impl Lexer{
    pub fn new() -> Lexer{
        Lexer{
            content: String::new(),
            source_filename: String::new(),
            cursor: 0,
            row: 1,
            col: 1,
            lexems: Vec::new()
        }
    }

    fn peek(self: &Self) -> Option<char> {
        return self.content.chars().nth(self.cursor)
    }

    fn chop(self: &mut Self) -> char{
        let ch = self.peek().unwrap();
        self.cursor += 1;
        self.col += 1;
        if ch == '\n'{
            self.row += 1;
            self.col = 1;
        }
        return ch;
    }

    fn seek_whitespace(self: &mut Self){
        while self.peek().unwrap().is_whitespace(){
            if self.peek().unwrap() == '\n'{
                break;
            }
            self.chop();
            if self.cursor >= self.content.len(){
                break;
            }
        }
    }

    fn chop_single(self: &mut Self) -> bool{
        if self.cursor >= self.content.len(){
            return false;
        }
        if self.peek().unwrap() == '\n'{
            let row = self.row;
            let col = self.col;
            let ch = self.chop();
            self.lexems.push(Lexem::new(ch.to_string(), LexemType::NewLine, row, col, self.source_filename.clone()));
            return true;
        }
        if SINGLE_LEXEMS.contains(&self.peek().unwrap()) {
            let row = self.row;
            let col = self.col;
            let ch = self.chop();
            self.lexems.push(Lexem::new(ch.to_string(), LexemType::Single,row, col, self.source_filename.clone()));
            return true;
        }
        return false;
    }

    fn chop_pattern(self: &mut Self) -> bool{
        if self.cursor >= self.content.len(){
            return false;
        }



        for pattern in OP_LEXEMS{
            let pattern = *pattern;
            if self.content.len() - self.cursor >= pattern.len(){
                if pattern == &self.content[self.cursor..self.cursor+pattern.len()]{
                    self.lexems.push(Lexem::new(pattern.to_string(), LexemType::Operator, self.row, self.col, self.source_filename.clone()));
                    for _ in 0..pattern.len(){
                        self.chop();
                    }
                    return true;
                }
            }
        }
        
        false
    }

    fn chop_word(self: &mut Self) -> bool{
        let mut lexem: String = String::new();

        let row = self.row;
        let col = self.col;

        while self.cursor < self.content.len() && (self.peek().unwrap().is_alphanumeric() || self.peek().unwrap() == '.'){

            lexem += self.chop().to_string().as_str();
        }

        if lexem.is_empty(){
            return false;
        }

        
        if lexem.starts_with("0x"){
            for (i, ch) in lexem.chars().skip(2).enumerate(){
                if !ch.is_ascii_hexdigit(){
                    println!("{}:{}:{} Expected hexlit got {}", self.source_filename, row, col+i+2, ch);
                    std::process::exit(1);
                }
                
            }
            self.lexems.push(Lexem::new(lexem.chars().skip(2).collect(), LexemType::Number { radix: 16 }, row,col, self.source_filename.clone()));
            return true;
        }
        
        if lexem.starts_with("0b"){
            for (i, ch) in lexem.chars().skip(2).enumerate(){
                if ch != '0' && ch != '1' {
                    println!("{}:{}:{} Expected binlit got {}", self.source_filename, row, col+i+2, ch);
                    std::process::exit(1);
                }
                
            }
            self.lexems.push(Lexem::new(lexem.chars().skip(2).collect(), LexemType::Number { radix: 2 }, row,col, self.source_filename.clone()));
            return true;
        }
        
        if lexem.chars().nth(0).unwrap().is_numeric(){
            for (i, ch) in lexem.chars().enumerate(){
                if !ch.is_numeric(){
                    println!("{}:{}:{} Expected number got {}", self.source_filename, row, col+i, ch);
                    std::process::exit(1);
                }

            }
            self.lexems.push(Lexem::new(lexem, LexemType::Number { radix: 10 }, row,col, self.source_filename.clone()));
            return true;
        }

        self.lexems.push(Lexem::new(lexem, LexemType::Ident, row,col, self.source_filename.clone()));

        return true;
    }

    fn chop_string(self: &mut Self) -> bool{
        let row = self.row;
        let col = self.col;

        let initial_cursor = self.cursor;
        let initial_row = self.row;
        let initial_col = self.col;

        if self.cursor >= self.content.len(){
            return false;
        }

        if self.peek().unwrap() != '\"' && self.peek().unwrap() != '\''{
            self.cursor = initial_cursor;
            self.row = initial_row;
            self.col = initial_col;
            return false;
        }

        self.chop();

        let mut value = String::new();

        while self.peek().unwrap() != '\"' && self.peek().unwrap() != '\''{
            if self.cursor >= self.content.len(){
                println!("{}:{}:{} Expected \" got end of file", self.source_filename, self.row, self.col);
                std::process::exit(1);
            }

            if self.peek().unwrap() == '\\'{
                self.chop();
                if self.cursor >= self.content.len(){
                    println!("{}:{}:{} Expected something got end of file", self.source_filename, self.row, self.col);
                    std::process::exit(1);
                }
                match self.chop(){
                    'n' => value += "\n",
                    '0' => value += "\0",
                    '\\' => value += "\\",
                    '\"' => value += "\"",
                    '\'' => value += "\'",
                     a  => {
                        println!("{}:{}:{} Unexpected character {}", self.source_filename, self.row, self.col, a);
                     }
                };

                continue;

            }

            value += self.chop().to_string().as_str();
        }

        self.chop();

        self.lexems.push(Lexem::new(value, LexemType::String, row, col, self.source_filename.clone() ));

        return true;
    }

    fn seek_comments(self: &mut Self){
        
        let initial_cursor = self.cursor;
        let initial_row = self.row;
        let initial_col = self.col;

        if self.content.len() - self.cursor >= 2{



            let test = self.chop().to_string() + self.chop().to_string().as_str();
            if test == "//"{
                if self.cursor >= self.content.len(){
                    self.cursor = initial_cursor;
                    self.row = initial_row;
                    self.col = initial_col;
                    return;
                }
                while self.peek().unwrap() != '\n'{
                    self.chop();
                    if self.cursor >= self.content.len(){
                        self.cursor = initial_cursor;
                        self.row = initial_row;
                        self.col = initial_col;
                        return;
                    }
                }
            }else{
                self.cursor = initial_cursor;
                self.row = initial_row;
                self.col = initial_col;
                return;
            }
        }

    }

    fn chop_lexem(self: &mut Self){

        if self.cursor >= self.content.len() {return}
        
        self.seek_whitespace();
        self.seek_comments();


        if self.chop_single() {return}

        if self.chop_pattern() {return}

        if self.chop_string() {return}

        if self.chop_word() {return}

        if self.cursor >= self.content.len(){
            return;
        }

        
        println!("{}:{}:{} unexpected character: \"{}\" at {}", self.source_filename, self.row, self.col, self.peek().unwrap(), self.cursor);
        std::process::exit(1);

    }

    pub fn lex<'a>(self: &mut Self, source_filename: &'a str, content: &'a str){
        self.cursor = 0;
        self.content = content.to_string();
        self.lexems.clear();
        self.source_filename = source_filename.to_string();
        while self.cursor < self.content.len(){
            self.chop_lexem();
        }
    }
}
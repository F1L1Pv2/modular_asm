use std::collections::HashMap;

use crate::{get_value_from_number_token, Lexem, LexemType};

use super::pseudo_instructions::PseudoInstructions;

#[derive(Debug, Clone)]
pub enum Token{
    Label{
        name: Lexem
    },
    Instruction{
        name: Lexem,
        args: Vec<Lexem>
    }
}

impl std::fmt::Display for Token{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match self{
            Token::Instruction { .. } => {write!(f, "Instruction")},
            Token::Label { .. } => {write!(f, "Label")}
        }
    }
}

fn fix_sub_label(last_label: &String, args: Vec<Lexem>) -> Vec<Lexem>{

    let mut new_args: Vec<Lexem> = Vec::new();

    for arg in args{
        match arg.ttype.clone() {

            LexemType::Closure { args } => {
                let mut new_arg: Vec<Lexem> = Vec::new();

                for arg in args{
                    let arg = *arg;
                    new_arg.push(arg);
                }

                let new_arg = fix_sub_label(last_label, new_arg);

                let mut args: Vec<Box<Lexem>> = Vec::new();

                for arg in new_arg{
                    args.push(Box::new(arg));
                }

                new_args.push(Lexem::new(arg.value.clone(), LexemType::Closure { args: [args[0].clone(), args[1].clone(), args[2].clone()] }, arg.row, arg.col, arg.filename.clone()));
            }

            LexemType::Ident => {

                if arg.value.starts_with("."){
                    new_args.push(Lexem::new(last_label.clone()+arg.value.as_str(), arg.ttype, arg.row, arg.col, arg.filename));
                }else{
                    new_args.push(Lexem::new(arg.value.clone(), arg.ttype, arg.row, arg.col, arg.filename));
                }

            }

            _ => {
                new_args.push(arg);
            }
        }
    }

    new_args
}


fn unpseudo_arg(arg: Lexem, pseudo_name: &Lexem, arg_hashmap: &HashMap<String, Lexem>) -> Lexem{
    
    match arg.ttype.clone(){
        LexemType::Closure { args } => {
            let mut new_args: Vec<Box<Lexem>> = Vec::new();
            for arg in args{
                let arg = *arg;
                new_args.push(Box::new(unpseudo_arg(arg, pseudo_name, arg_hashmap)));
            }

            Lexem::new(arg.value, LexemType::Closure { args: [new_args[0].clone(),new_args[1].clone(),new_args[2].clone()] }, arg.row, arg.col, arg.filename)
        }
        _ => {
            let arg_name = arg.value.clone();
            let new_arg = match arg_hashmap.get(&arg_name){
                Some(a) => a.clone(),
                None => Lexem::new(arg_name, arg.ttype, pseudo_name.row, pseudo_name.col, pseudo_name.filename.clone())
            };

            new_arg
        }
    }
}

fn eval_closure(arg: Lexem, args: [Box<Lexem>; 3]) -> Lexem{
    
    let lhs = *args[0].clone();
    let lhs = match lhs.ttype.clone(){
        LexemType::Closure { args } => {
            eval_closure( lhs, args)
        }
        _ => lhs
    };

    match lhs.ttype{
        LexemType::Number { .. } => {}
        LexemType::Ident { .. } => {
            println!("{}:{}:{} Use of undeclared label {}", lhs.filename, lhs.row, lhs.col, lhs.value);
            std::process::exit(1);
        }
        _ => {
            println!("{}:{}:{} Expected Number got {}", lhs.filename, lhs.row, lhs.col, lhs.ttype);
            std::process::exit(1);
        }
    }

    let rhs = *args[2].clone();
    let rhs = match rhs.ttype.clone(){
        LexemType::Closure { args } => {
            eval_closure(rhs, args)
        }
        _ => rhs
    };

    match rhs.ttype{
        LexemType::Number { .. } => {}
        LexemType::Ident { .. } => {
            println!("{}:{}:{} Use of undeclared label {}", rhs.filename, rhs.row, rhs.col, rhs.value);
            std::process::exit(1);
        }
        _ => {
            println!("{}:{}:{} Expected Number got {}", rhs.filename, rhs.row, rhs.col, rhs.ttype);
            std::process::exit(1);
        }
    }
    

    let lhs = get_value_from_number_token(&lhs);
    let rhs = get_value_from_number_token(&rhs);

    let op = *args[1].clone();
    if op.ttype != LexemType::Operator{
        println!("{}:{}:{} Expected Operator got {}", op.filename, op.row, op.col, op.ttype);
        std::process::exit(1);
    }

    let ret_val: usize;

    match op.value.as_str(){
        "+" => {ret_val = lhs+rhs},
        "-" => {ret_val = lhs-rhs},
        "*" => {ret_val = lhs*rhs},
        "/" => {ret_val = lhs/rhs},
        "&" => {ret_val = lhs&rhs},
        "|" => {ret_val = lhs|rhs},
        "^" => {ret_val = lhs^rhs},
        "<<" => {ret_val = lhs<<rhs},
        ">>" => {ret_val = lhs>>rhs},
        _ => {
            println!("{}:{}:{} Invalid Operator {}",op.filename, op.row, op.col, op.value);
            std::process::exit(1);
        }
    }

    Lexem::new(format!("{}", ret_val), LexemType::Number { radix: 10 }, arg.row, arg.col, arg.filename)
}

pub struct Parser{
    cursor: usize,
    lexems: Vec<Lexem>,
    pub tokens: Vec<Token>
}

impl Parser{
    pub fn new() -> Parser{
        Parser{
            cursor: 0,
            lexems: Vec::new(),
            tokens: Vec::new()
        }
    }


    fn peek_lexem(self: &Self) -> Option<Lexem>{

        if self.cursor >= self.lexems.len(){
            return None;
        }

        Some(self.lexems[self.cursor].clone())
    }

    fn chop_newline(self: &mut Self){
        while self.peek_lexem().unwrap().ttype == LexemType::NewLine{
            self.chop_lexem();
            if self.cursor >= self.lexems.len(){
                return;
            }
        }
    }

    fn chop_lexem(self: &mut Self) -> Lexem{
        let lexem = self.peek_lexem().unwrap();
        self.cursor += 1;
        return lexem;
    }

    fn parse_lexem_label(self: &mut Self) -> bool{
        
        let initial_cursor = self.cursor;
        
        self.chop_newline();

        if self.cursor >= self.lexems.len(){
            self.cursor = initial_cursor;
            return false;
        }


        if self.peek_lexem().unwrap().ttype != LexemType::Ident{
            self.cursor = initial_cursor;
            return false;
        }

        let label_name = self.chop_lexem();

        if self.cursor >= self.lexems.len(){
            self.cursor = initial_cursor;
            return false;
        }

        if self.peek_lexem().unwrap().value != ":"{
            self.cursor = initial_cursor;
            return false;
        }

        self.chop_lexem();

        self.tokens.push(Token::Label { name: label_name });

        return true;
    }

    fn parse_arg(self: &mut Self) -> Lexem{
        let lexem = self.chop_lexem();

        if lexem.value == "("{
            
            
            let lhs = Box::new(self.parse_arg());
            let operator = Box::new(self.parse_arg());
            let rhs = Box::new(self.parse_arg());
            let lexem = Lexem::new("Closure".to_string(), LexemType::Closure { args: [lhs, operator, rhs] }, lexem.row, lexem.col, lexem.filename);

            let test = self.chop_lexem();

            if test.value == ")"{
                return lexem;
            }else{
                println!("{}:{}:{} Expected \")\" got \"{}\"", test.filename, test.row, test.col, test.value);
                std::process::exit(1);
            }
        }else{
            return lexem;
        }
    }

    fn parse_args(self: &mut Self) -> Option<Vec<Lexem>>{

        let mut args: Vec<Lexem> = Vec::new();

        // let arg_types = &[LexemType::Number{radix: 0} ,LexemType::Ident, LexemType::Register];

        if self.cursor >= self.lexems.len(){
            return  Some(Vec::new());
        }

        if self.peek_lexem().unwrap().ttype == LexemType::NewLine{
            return Some(Vec::new());
        }


        args.push(self.parse_arg());

        if self.cursor >= self.lexems.len(){
            return Some(args);
        }

        while self.cursor < self.lexems.len() && self.peek_lexem().unwrap().value == ","{
            let x = self.chop_lexem();

            if self.cursor >= self.lexems.len(){
                println!("{}:{}:{} Expected arg got end of file", x.filename, x.row, x.col+1);
                std::process::exit(1);
            }

            args.push(self.parse_arg());

            if self.cursor < self.lexems.len() && self.peek_lexem().unwrap().ttype == LexemType::NewLine{
                break;
            }

        }

        return Some(args);
    }

    fn parse_lexem_instruction(self: &mut Self) -> bool{

        
        let initial_cursor = self.cursor;
        
        self.chop_newline();

        if self.cursor >= self.lexems.len(){
            self.cursor = initial_cursor;
            return false;
        }
        
        if self.peek_lexem().unwrap().ttype != LexemType::Ident{
            self.cursor = initial_cursor;
            return false;
        }

        let name = self.chop_lexem();

        let args = match self.parse_args(){
            Some(a) => a,
            None => {
                self.cursor = initial_cursor;
                return false;
            }
        };

        self.tokens.push(Token::Instruction { name, args });


        return true;
    }

    fn parse_token(self: &mut Self){
        if self.parse_lexem_label(){return}
        
        if self.parse_lexem_instruction(){return}
        
        self.chop_newline();

        if self.cursor >= self.lexems.len(){
            return;
        }

        let lexem = self.peek_lexem().unwrap();
        println!("{}:{}:{} got unexpected token {}", lexem.filename, lexem.row, lexem.col, lexem.value);
        // dbg!(&self.tokens);
        std::process::exit(1);
    }

    pub fn first_stage_parse<'a>(self: &mut Self, lexems: &Vec<Lexem>){
        self.lexems = lexems.clone();
        self.cursor = 0;

        self.tokens.clear();
        
        while self.cursor < self.lexems.len(){
            self.parse_token()
        }
    }


    fn convert_pseudo_instructions(self: &mut Self){
        let pseudo_instructions = PseudoInstructions::initialize();

        let mut after_pseudo: Vec<Token> = Vec::new();

        for token in self.tokens.iter_mut(){
            match token{
                Token::Instruction { name, args } =>{
                    if pseudo_instructions.keys().collect::<Vec<&String>>().contains(&&name.value){
                        let mut arg_hashmap: HashMap<String, Lexem> = HashMap::new();

                        let pseudo = match pseudo_instructions.get(name.value.as_str()){
                            Some(a) => a,
                            None => {
                                println!("{}:{}:{} Pasrser pseudo_instructions: Impossible Error", name.filename, name.row, name.col);
                                std::process::exit(1);
                            }
                        }.clone();

                        if args.len() != pseudo.0.len(){
                            println!("{}:{}:{} Expects {} ammount of args got {}", name.filename, name.row, name.col, pseudo.0.len(), args.len());
                        }

                        for (i, arg) in pseudo.0.iter().enumerate(){
                            arg_hashmap.insert(arg.clone(), args[i].clone());
                        }

                        let pseudo_name = name;

                        for token in pseudo.1{
                            match token{
                                Token::Instruction { name, args } => {
                                    let mut new_args: Vec<Lexem> = Vec::new();
                                    for arg in args{
                                        new_args.push(unpseudo_arg(arg, pseudo_name, &arg_hashmap));
                                    }
                                    after_pseudo.push(Token::Instruction { name, args: new_args });
                                    // after_pseudo.push(Token::Instruction { name: Lexem::new(name.value, name.ttype, pseudo_name.row, pseudo_name.col, pseudo_name.filename.clone()), args: new_args });
                                }
                                Token::Label { name } => {
                                    println!("{}:{}:{} Currently labels are not possible inside pseudo instruction: {}", pseudo_name.filename, pseudo_name.row, pseudo_name.col, name.value);
                                }
                            }
                        }


                    }else{
                        after_pseudo.push(Token::Instruction {  name: name.clone(), args: args.clone() });
                    }
                },
                Token::Label { name } => {
                    after_pseudo.push(Token::Label { name: name.clone() });
                }
            }
        }
        self.tokens = after_pseudo;
    }

    fn discover_labels(self: &mut Self) -> (Vec<Token>, HashMap<String, usize>) {
        let mut origin: usize = 0;
        self.cursor = 0;

        let mut cleaned_tokens: Vec<Token> = Vec::new();

        let mut labels: HashMap<String, usize> = HashMap::new();

        let mut last_label = String::new();
        
        for token in self.tokens.iter(){
            match token{
                Token::Instruction { name, args } => {
                    
                    let name = name.clone();
                    let args = args.clone();


                    match name.value.to_lowercase().as_str() {
                        "org" => {
                            if args.len() != 1{
                                println!("{}:{}:{} you need to provide addr", name.filename, name.row, name.col);
                                std::process::exit(1);
                            }

                            let arg = args[0].clone();
                            if !matches!(arg.ttype, LexemType::Number { .. }){
                                println!("{}:{}:{} Expected number got {}", arg.filename, arg.row, arg.col, arg.ttype);
                                std::process::exit(1);
                            }

                            origin = get_value_from_number_token(&arg);
                            self.cursor = 0;
                        }

                        "dw" => {
                        
                            let mut to_add = 0;
    
                            for arg in args.iter(){
                                match arg.ttype{
                                    LexemType::Ident => to_add += 1,
                                    LexemType::String => to_add += arg.value.len()*2,
                                    LexemType::Number { .. } => to_add += 1,
                                    _ => {
                                        println!("{}:{}:{} Unexpected token {}", arg.filename, arg.row, arg.col, arg.ttype);
                                    }
                                }
                            }
    
                            cleaned_tokens.push(Token::Instruction { name, args });
                            self.cursor += to_add;
                        }

                        "dd" => {
                        
                            let mut to_add = 0;
    
                            for arg in args.iter(){
                                match arg.ttype{
                                    LexemType::Ident => to_add += 2,
                                    LexemType::String => to_add += arg.value.len()*4,
                                    LexemType::Number { .. } => to_add += 2,
                                    _ => {
                                        println!("{}:{}:{} Unexpected token {}", arg.filename, arg.row, arg.col, arg.ttype);
                                    }
                                }
                            }
    
                            cleaned_tokens.push(Token::Instruction { name, args });
                            self.cursor += to_add;
                        }

                        "dq" => {
                        
                            let mut to_add = 0;
    
                            for arg in args.iter(){
                                match arg.ttype{
                                    LexemType::Ident => to_add += 4,
                                    LexemType::String => to_add += arg.value.len()*8,
                                    LexemType::Number { .. } => to_add += 4,
                                    _ => {
                                        println!("{}:{}:{} Unexpected token {}", arg.filename, arg.row, arg.col, arg.ttype);
                                    }
                                }
                            }
    
                            cleaned_tokens.push(Token::Instruction { name, args });
                            self.cursor += to_add;
                        }

                        _ => {
                            // cleaned_tokens.push(Token::Instruction { name, args });

                            cleaned_tokens.push( Token::Instruction{ name, args: fix_sub_label(&last_label, args)});

                            self.cursor += 1;
                        }

                    }
                }
                Token::Label { name } => {

                    match labels.get(&name.value) {
                        Some(_) => {
                            println!("{}:{}:{} Label already defined {}", name.filename, name.row, name.col, name.value);
                            std::process::exit(1)
                        }
                        None => {

                            if name.value.starts_with("."){
                                labels.insert(last_label.clone()+name.value.as_str(), origin+self.cursor);
                            }else{
                                last_label = name.value.clone();
                                labels.insert(last_label.clone(), origin+self.cursor);
                            }

                        }
                    }

                }
            }
        }

        (cleaned_tokens, labels)
    }

    fn fix_args(self: &mut Self, labels: &HashMap<String, usize>, args: &mut Vec<Lexem>) -> Vec<Lexem>{
        let mut new_args: Vec<Lexem> = Vec::new();
        for arg in args{
            match arg.ttype.clone() {

                LexemType::Closure { args } =>{
                    
                    let mut nargs = Vec::new();
                    
                    for arg in args{
                        nargs.push(*arg);
                    }
                    let args = self.fix_args(labels, &mut nargs);

                    let mut nargs = Vec::new();

                    for arg in args{
                        nargs.push(Box::new(arg));
                    }

                    new_args.push(Lexem::new(arg.value.clone(), LexemType::Closure { args: [nargs[0].clone(),nargs[1].clone(),nargs[2].clone()] }, arg.row, arg.col, arg.filename.clone()));
                }

                LexemType::Ident =>{
                    match labels.get(&arg.value){
                        Some(x) => {
                            new_args.push(Lexem::new(format!("{}",x),LexemType::Number { radix: 10 },arg.row,arg.col, arg.filename.clone()));
                        },
                        None => {
                            new_args.push(arg.clone());
                            // println!("{}:{}:{} use of undeclared label {}", arg.filename, arg.row, arg.col, arg.value);
                            // std::process::exit(1);
                        }
                    };
                    

                }

                _ => new_args.push(arg.clone())
            }
        }

        new_args
    }

    fn calculate_labels(self: &mut Self){
        let (mut cleaned_tokens, labels) = self.discover_labels();

        for arg in cleaned_tokens.iter_mut(){
            match arg{
                Token::Instruction { name: _, args } =>{
                    *args = self.fix_args(&labels, args);
                }
                Token::Label { .. } => {
                    println!("Internal error labels shouldve been removed in this stage");
                    std::process::exit(1);
                }
            }
        }

        self.tokens = cleaned_tokens;
    }

    // fn colapse_closure(self: &mut Self, arg: Lexem){

    // }

    fn colapse_closures(self: &mut Self){
        
        let mut new_tokens = Vec::new();
        
        for token in self.tokens.iter(){
            match token{
                Token::Instruction { name, args } => {

                    let mut new_args: Vec<Lexem> = Vec::new();

                    for arg in args{
                        match arg.ttype.clone(){

                            LexemType::Closure { args } => {
                                new_args.push(eval_closure(arg.clone(), args))
                            }

                            _ => {
                                new_args.push(arg.clone());
                            }
                        }
                    }

                    new_tokens.push(Token::Instruction { name: name.clone(), args: new_args })

                }
                Token::Label { name } => {
                    println!("{}:{}:{} This shouldnt exist now", name.filename, name.row, name.col);
                    std::process::exit(1);
                }
            }
        }

        self.tokens = new_tokens;
    }

    pub fn parse<'a>(self: &mut Self, lexems: &Vec<Lexem>){
        
        self.first_stage_parse(lexems);

        self.convert_pseudo_instructions();

        self.calculate_labels();

        self.colapse_closures();

    }
}

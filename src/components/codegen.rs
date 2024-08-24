use std::collections::HashMap;

use crate::{InstructionPart, Lexem, LexemType, Token, TYPES};

#[derive(Debug)]
pub struct CodeGen<'a>{
    tokens: &'a[Token],
    instruction_set: &'a HashMap<&'static str,Vec<InstructionPart>>,
    pub bytes: Vec<u8>
}

pub fn get_value_from_number_token<'a>(lexem: &Lexem) -> usize{
    match lexem.ttype{
        LexemType::Number { radix } => {usize::from_str_radix(&lexem.value, radix as u32).unwrap()}
        _ => {
            println!("{}:{}:{} Expected number got {}", lexem.filename, lexem.row, lexem.col,lexem.ttype);
            std::process::exit(1);
        }
    }
}

impl CodeGen<'_>{
    pub fn new<'a>(tokens: &'a[Token], instruction_set: &'a HashMap<&'static str,Vec<InstructionPart>>) -> CodeGen<'a>{
        CodeGen{
            tokens,
            instruction_set,
            bytes: Vec::new()
        }
    }

    pub fn str_to_bytes(self: &Self, str: &String) -> Vec<u8>{
        if str.len() == 0{
            return vec![];
        }

        let ret: usize = usize::from_str_radix(str, 2).unwrap();
        if str.len() <= 8{
            return vec![ret as u8];
        }

        if str.len() <= 16{
            return (ret as u16).to_be_bytes().to_vec();
        }

        if str.len() <= 32{
            return (ret as u32).to_be_bytes().to_vec();
        }

        if str.len() <= 64{
            return (ret as u64).to_be_bytes().to_vec();
        }

        eprintln!("too big");
        std::process::exit(1);
    }

    pub fn gen(self: &mut Self){

        for token in self.tokens.iter(){
            match token{
                Token::Instruction { name, args } => {
                    match name.value.as_str(){
                        "org" => {
                            println!("Org: Error in parser");
                            std::process::exit(1);
                        }

                        "db" => {
                            if args.len() == 0{
                                println!("{}:{}:{} No data was provided", name.filename, name.row, name.col+name.value.len());
                                std::process::exit(1);
                            }
                            
                            for arg in args{
                                match arg.ttype{
                                    LexemType::Number { radix } => {
                                        let b = ((usize::from_str_radix(&arg.value, radix as u32).unwrap() & 0xFFFF ) as u8).to_be_bytes();
                                        for b in b{
                                            self.bytes.push(b);
                                        }
                                    },
                                    LexemType::String => {
                                        for ch in arg.value.chars(){
                                            self.bytes.push(ch as u8);
                                        }
                                    },
                                    _ => {
                                        println!("{}:{}:{} Unexpected lexem {}", arg.filename, arg.row, arg.col, arg.ttype);
                                        std::process::exit(1);
                                    }
                                }
                            }
                        }

                        "dw" => {
                            if args.len() == 0{
                                println!("{}:{}:{} No data was provided", name.filename, name.row, name.col+name.value.len());
                                std::process::exit(1);
                            }

                            for arg in args{
                                match arg.ttype{
                                    LexemType::Number { radix } => {
                                        let b = ((usize::from_str_radix(&arg.value, radix as u32).unwrap() & 0xFFFF ) as u16).to_be_bytes();
                                        for b in b{
                                            self.bytes.push(b);
                                        }
                                    },
                                    LexemType::String => {
                                        for ch in arg.value.chars(){
                                            self.bytes.push(0);
                                            self.bytes.push(ch as u8);
                                        }
                                    },
                                    _ => {
                                        println!("{}:{}:{} Unexpected lexem {}", arg.filename, arg.row, arg.col, arg.ttype);
                                        std::process::exit(1);
                                    }
                                }
                            }

                        }

                        "dd" => {
                            if args.len() == 0{
                                println!("{}:{}:{} No data was provided", name.filename, name.row, name.col+name.value.len());
                                std::process::exit(1);
                            }
    
                            for arg in args{
                                match arg.ttype{
                                    LexemType::Number { radix } => {
                                        let b = ((usize::from_str_radix(&arg.value, radix as u32).unwrap() & 0xFFFFFFFF ) as u32).to_be_bytes();
                                        for b in b{
                                            self.bytes.push(b);
                                        }
                                    },
                                    LexemType::String => {
                                        for ch in arg.value.chars(){
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(ch as u8);
                                        }
                                    },
                                    _ => {
                                        println!("{}:{}:{} Unexpected lexem {}", arg.filename, arg.row, arg.col, arg.ttype);
                                        std::process::exit(1);
                                    }
                                }
                            }
                        }

                        "dq" => {
                            if args.len() == 0{
                                println!("{}:{}:{} No data was provided", name.filename, name.row, name.col+name.value.len());
                                std::process::exit(1);
                            }

                            for arg in args{
                                match arg.ttype{
                                    LexemType::Number { radix } => {
                                        let b = ((usize::from_str_radix(&arg.value, radix as u32).unwrap() & 0xFFFFFFFFFFFFFFFF ) as u32).to_be_bytes();
                                        for b in b{
                                            self.bytes.push(b);
                                        }
                                    },
                                    LexemType::String => {
                                        for ch in arg.value.chars(){
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(0);
                                            self.bytes.push(ch as u8);
                                        }
                                    },
                                    _ => {
                                        println!("{}:{}:{} Unexpected lexem {}", arg.filename, arg.row, arg.col, arg.ttype);
                                        std::process::exit(1);
                                    }
                                }
                            }

                        }

                        _ => {
                            let instruction = match self.instruction_set.get(&name.value.as_str()){
                                Some(a) => a,
                                None => {
                                    println!("{}:{}:{} Unknown instruction {}", name.filename, name.row, name.col, name.value);
                                    std::process::exit(1);
                                }
                            }.as_slice();

                            let mut args = args.clone();

                            let mut bits_str = String::new();

                            
                            for part in instruction{
                                match part{
                                    InstructionPart::Const { val } => {
                                        bits_str+=val;
                                    },

                                    InstructionPart::Type { val, size } => {
                                        if args.len() == 0{
                                            println!("{}:{}:{} Expected Argument", name.filename, name.row, name.col+name.value.len());
                                            std::process::exit(1);
                                        }
                                        let arg = args.remove(0);
                                        if !matches!(arg.ttype, LexemType::Ident){
                                            println!("{}:{}:{} Expected ident got {}", arg.filename, arg.row, arg.col, arg.ttype);
                                            std::process::exit(1);
                                        }
                                        let type_val = val.to_uppercase();

                                        let type_hashmap = match TYPES.get(&type_val){
                                            Some(a) => a,
                                            None => {
                                                println!("{}:{}:{} following type {} doesn't exist", arg.filename, arg.row, arg.col, type_val);
                                                std::process::exit(1);
                                            }
                                        };


                                        let val = match type_hashmap.get(arg.value.to_lowercase().as_str()){
                                            Some(a) => a,
                                            None => {
                                                println!("{}:{}:{} type {} doesn't have {}", arg.filename, arg.row, arg.col, type_val, arg.value);
                                                std::process::exit(1);
                                            }
                                        };

                                        let val = format!("{:b}", val);

                                        bits_str+="0".repeat(*size - val.len()).as_str();
                                        bits_str+=val.as_str();
                                    }
                                    
                                    InstructionPart::Imm { size } => {
                                        if args.len() == 0{
                                            println!("{}:{}:{} Expected Immediate", name.filename, name.row, name.col+name.value.len());
                                            std::process::exit(1);
                                        }
                                        let arg = args.remove(0);
                                        
                                        if arg.ttype == LexemType::Ident{
                                            println!("{}:{}:{} Use of undeclared label {}", arg.filename, arg.row, arg.col, arg.value);
                                            std::process::exit(1);
                                        }

                                        let val = get_value_from_number_token(&arg);
                                        
                                        
                                        let val = format!("{:b}", val);
                                        
                                        if val.len() > *size{
                                            println!("{}:{}:{} Number is too big {}", arg.filename, arg.row, arg.col, arg.value);
                                            std::process::exit(1);
                                        }

                                        bits_str+="0".repeat(*size - val.len()).as_str();
                                        
                                        bits_str += val.as_str();
                                    }

                                    InstructionPart::Extra { size } => {
                                        if args.len() == 0{
                                            bits_str += "0".repeat(*size).as_str();
                                            continue;
                                        }
                                        
                                        let arg = args.remove(0);
                                        
                                        let val = get_value_from_number_token(&arg);
                                        
                                        
                                        let val = format!("{:b}", val);
                                        
                                        if val.len() > *size{
                                            println!("{}:{}:{} Number is too big {}", arg.filename, arg.row, arg.col, arg.value);
                                            std::process::exit(1);
                                        }
                                        
                                        bits_str+="0".repeat(*size - val.len()).as_str();
                                        bits_str += val.as_str();
                                    }
                                    
                                }
                            }

                            let mut that_bytes = self.str_to_bytes(&bits_str);
                            self.bytes.append(&mut that_bytes);
                        }
                    }

                },

                Token::Label { name } => {
                    println!("{}:{}:{} Error in parser", name.filename, name.row, name.col);
                    std::process::exit(1);
                }

            }

        }

    }
}


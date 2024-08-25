use std::collections::HashMap;

use crate::{INSTRUCTIONS, TYPES};

#[derive(Debug, Clone)]
pub enum InstructionPart{
    Const{
        val: String
    },
    Imm{
        size: usize,
    },
    Type{
        val: String,
        size: usize,
    },
    Extra{
        size: usize
    },
}

#[derive(Debug)]
pub struct InstructionsLexer{
    cursor: usize,
    pub instructions: HashMap<&'static str,Vec<InstructionPart>>
}


impl InstructionsLexer{

    pub fn new() -> InstructionsLexer{
        InstructionsLexer{
            cursor: 0,
            instructions: HashMap::new()
        }
    }

    fn peek(self: &mut Self, str: &'static str) -> Option<char>{
        str.chars().nth(self.cursor).clone()
    }

    fn chop(self: &mut Self, str: &'static str) -> char{
        let x = self.peek(str).unwrap();
        self.cursor += 1;
        x
    }

    fn chop_white_space(self: &mut Self, str: &'static str){
        while self.cursor < str.len() && self.peek(str).unwrap().is_whitespace(){
            self.chop(str);
        }
    }

    pub fn get_instruction_size(self: &Self, name: &str) -> usize{
        match self.instructions.get(name){
            Some(a) => {
                let mut instruction_size: usize = 0;
                for instruction_part in a{
                    match instruction_part {
                        InstructionPart::Const { val } => {
                            instruction_size += val.len();
                        }

                        InstructionPart::Extra { size } => {
                            instruction_size += size;
                        }

                        InstructionPart::Imm { size } => {
                            instruction_size += size;
                        }
                        InstructionPart::Type { val: _, size } => {
                            instruction_size += size;
                        }
                    }
                }

                let mut return_size = instruction_size / 8;

                if instruction_size % 8 != 0{
                    return_size += 1;
                }

                return_size
            }
            None => return usize::MAX
        }
    }

    fn chop_ones_zeroes(self: &mut Self, str: &'static str) -> Option<InstructionPart>{
        
        let mut val = String::new();

        let initial_cursor = self.cursor;
        
        while self.cursor < str.len() && (self.peek(str).unwrap().is_whitespace() || (self.peek(str).unwrap() == '0' || self.peek(str).unwrap() == '1' )){
            self.chop_white_space(str);
            while self.cursor < str.len() && (self.peek(str).unwrap() == '0' || self.peek(str).unwrap() == '1' ){
                val += self.chop(str).to_string().as_str();
            }
        }

        if val.len() == 0{
            self.cursor = initial_cursor;
            return None;
        }

        Some(InstructionPart::Const { val: val.clone()})

        
    }

    fn chop_curly(self: &mut Self,name: &'static str, str: &'static str) -> Option<InstructionPart>{

        let initial_cursor = self.cursor;

        self.chop_white_space(str);

        if self.peek(str).unwrap() != '{'{
            self.cursor = initial_cursor;
            return None
        }

        self.chop(str);

        let mut ttype = String::new();

        self.chop_white_space(str);

        while self.cursor < str.len() && self.peek(str).unwrap().is_alphabetic(){
            ttype += self.chop(str).to_string().as_str();
        }

        if ttype.len() == 0{
            println!("Instruction Lexer \"{}\": You need to provide type for types", name);
            std::process::exit(1);
        }

        self.chop_white_space(str);

        let mut size = String::new();

        while self.cursor < str.len() && self.peek(str).unwrap().is_numeric(){
            size += self.chop(str).to_string().as_str();
        }

        if size.len() == 0{
            println!("Instruction Lexer \"{}\": You need to provide size for types", name);
            std::process::exit(1);
        }

        self.chop_white_space(str);

        let ch = self.chop(str);

        if ch != '}'{
            println!("Instruction Lexer \"{}\": expected closed curly got {}", name, ch);
            std::process::exit(1);
        }

        let size = usize::from_str_radix(&size, 10).unwrap();

        match ttype.to_uppercase().as_str(){
            "IMM" => {
                return Some(InstructionPart::Imm { size });
            },
            "E" => {
                return Some(InstructionPart::Extra { size });
            }
            _ => {

                if !TYPES.contains_key(ttype.to_uppercase().as_str()){
                    println!("Instruction Lexer \"{}\": Unknown type {}", name, ttype.to_uppercase());
                    std::process::exit(1);
                }

                return Some(InstructionPart::Type { val: ttype.to_uppercase().clone(), size });
            }
        }
    }

    fn lex_instruction(self: &mut Self, name: &'static str, instruction: &'static str) -> Vec<InstructionPart>{
        
        let mut parts: Vec<InstructionPart> = Vec::new();
        
        self.cursor = 0;
        while self.cursor < instruction.len(){

            match self.chop_ones_zeroes(instruction){
                Some(x) => {
                    parts.push(x);
                    continue;
                }
                None => {}
            }

            match self.chop_curly(name, instruction){
                Some(x) => {
                    parts.push(x);
                    continue;
                }
                None => {}
            }

            println!("instruction_lexer: unknown character: \"{}\"", self.peek(instruction).unwrap());
            dbg!(parts);
            std::process::exit(1);
        }


        parts
    }

    pub fn lex_instructions(self: &mut Self){
        for (name, instruction) in INSTRUCTIONS.entries(){
            let name = *name;
            let instruction = *instruction;

            let instruction = self.lex_instruction(name, instruction);
            self.instructions.insert(name, instruction);
        }
    }
}
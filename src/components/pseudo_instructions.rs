use std::collections::HashMap;

use crate::{Token, Parser, Lexer, PSEUDO_INSTRUCTIONS};

#[derive(Debug)]
pub struct PseudoInstructions{}

impl PseudoInstructions{
    pub fn initialize() -> HashMap<String, (Vec<String>,Vec<Token>)>{
        let mut pseudo_instructions_lexer: Lexer = Lexer::new();
        let mut pseudo_instructions_parser: Parser = Parser::new(None);

        let mut pseudo_instructions: HashMap<String, (Vec<String>,Vec<Token>)> = HashMap::new();
        
        for (name, code) in PSEUDO_INSTRUCTIONS.entries(){
            let mut p_args = Vec::new();
            let name = *name;
            let code = *code;

            let pure_name = name.split(" ").collect::<Vec<&str>>()[0];

            let source_filename = "PSEUDO_INSTRUCTION_NAME_".to_string() + pure_name.to_uppercase().as_str();

            pseudo_instructions_lexer.lex(&source_filename, name);

            pseudo_instructions_parser.first_stage_parse(&pseudo_instructions_lexer.lexems);

            if pseudo_instructions_parser.tokens.len() > 1{
                println!("PSEUDO_INSTRUCTIONS: You can only have one name per pseudoinstruction");
                std::process::exit(1);
            }

            match pseudo_instructions_parser.tokens[0].clone(){
                Token::Instruction { name: _, args } => {
                    for arg in args{
                        p_args.push(arg.value);
                    }
                }
                Token::Label { .. } => {
                    println!("PSEUDO_INSTRUCTIONS: You can only define instruction");
                    std::process::exit(1);
                }
            }


            let source_filename = "PSEUDO_INSTRUCTION_CODE_".to_string() + pure_name.to_uppercase().as_str();

            pseudo_instructions_lexer.lex(source_filename.as_str(), code);

            pseudo_instructions_parser.first_stage_parse(&pseudo_instructions_lexer.lexems);

            pseudo_instructions.insert(pure_name.to_string(), (p_args.clone(),pseudo_instructions_parser.tokens.clone()));
        }

        
        pseudo_instructions

    }
}
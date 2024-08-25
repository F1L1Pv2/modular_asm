/*
 made by:
  ___ _ _    _ ___      ___ 
 | __/ | |  / | _ \__ _|_  )
 | _|| | |__| |  _/\ V // / 
 |_| |_|____|_|_|   \_//___|
*/

use std::io::Write;
use std::path::Path;
use std::{fs::File, io::Read};

mod components;
mod config;
use components::lexer::*;
use components::parser::*;
use components::codegen::*;
use config::*;
use components::instruction_lexer::*;
use components::pseudo_instructions::PseudoInstructions;

fn main() {
    let mut instruction_lexer: InstructionsLexer = InstructionsLexer::new();

    instruction_lexer.lex_instructions();

    let mut args = std::env::args();

    let filename = args.next().unwrap();

    let source_filename = match args.next(){
        Some(n) => {n},
        None => {
            println!("{}: Source Filename wasn't provided", filename);
            std::process::exit(1);
        }
    };

    let path = Path::new(&source_filename);

    let mut file = File::open(path).unwrap();

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    
    let mut lexer: Lexer = Lexer::new();
    
    lexer.lex(&source_filename, &content);
    
    let mut parser: Parser = Parser::new(Some(PseudoInstructions::initialize()));
    
    parser.parse(&lexer.lexems, &instruction_lexer);
    
    let mut codegen: CodeGen = CodeGen::new(&parser.tokens, &instruction_lexer.instructions);

    codegen.gen();

    let output_str = match path.extension(){
        Some(a) => path.to_str().unwrap().replace(a.to_str().unwrap(),"bin"),
        None => path.to_str().unwrap().to_string() + ".bin"
    };

    let mut file = File::create(&output_str).unwrap();

    let _ =file.write(&codegen.bytes);

    println!("Assembled file: {} ({} bytes)", output_str, codegen.bytes.len());

    dbg!()
    

}

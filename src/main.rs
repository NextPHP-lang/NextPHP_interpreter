mod scanner;
mod token;
mod tokentype;
mod object;
mod ast;
mod error;
mod parser;
mod interpreter;

use std::env::args;
use std::fs;
use crate::error::ScrapError;
use crate::error::ScrapError::RuntimeError;
use crate::interpreter::Interpreter;

use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::Token;


fn main(){
    let input: Vec<String> = args().collect();
    if input.len() < 3 {
        error::ScrapError::error(ScrapError::RuntimeError, "too few arguments", 1, file!())
    }
    if input[1] == "main.rs" {
        run_file(input[2].clone());
    }
struct Scrap {}

fn run_file(source: String) {
    // println!("{:?}", source);
    let input = fs::read_to_string(source).expect("missing file");
    run(input)
}
fn run(input: String) {
    let mut scanner = Scanner::new(input);
    let tokens =  &scanner.scan_tokens();
    for token in &scanner.tokens {
        println!("{:?} \n", token);
    };

    let mut parser = Parser::new(scanner.tokens);
    parser.parse();

    let mut interpreter = Interpreter::new(parser.expressions);
    interpreter.evaluate();


}}
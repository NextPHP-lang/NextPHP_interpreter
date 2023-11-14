mod scanner;
mod token;
mod tokentype;
mod object;
mod ast;
mod error;
// mod parser;
mod interpreter;
mod evaluator;
mod parser_2;


use std::env::args;
use std::fs;
use crate::error::ScrapError;
use crate::error::ScrapError::RuntimeError;
use crate::evaluator::Evaluator;
use crate::interpreter::Interpreter;
use crate::parser_2::Parser;
// use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::Token;


fn main(){
    std::env::set_var("RUST_BACKTRACE", "5");
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
    let mut evaluator = Evaluator::new(parser.expressions);
    evaluator.start();

    // let mut interpreter = Interpreter::new(parser.expressions);
    // interpreter.evaluate();


}}
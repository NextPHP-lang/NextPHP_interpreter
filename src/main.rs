mod scanner;
mod token;
mod tokentype;
mod object;

use std::env::args;
use std::fs;
use crate::scanner::Scanner;
use crate::token::Token;


fn main(){
    let input: Vec<String> = args().collect() ;
    run_file(input[2].clone());
}
struct NextPHP {}

fn run_file(source: String) {
    println!("{:?}", source);
    let input = fs::read_to_string(source)
        .expect("expected file");
    run(input);
}
fn run(input: String) {
    let mut scanner = Scanner::new(input);
    let tokens =  scanner.scan_tokens();
    for token in scanner.tokens {
        println!("{:?}", token);
    };
}
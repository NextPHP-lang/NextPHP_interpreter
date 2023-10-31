mod scanner;
mod token;
mod tokentype;

use std::env::args;
use std::fs;
use crate::scanner::Scanner;
use crate::token::Token;


fn main(){
    let input = args().collect() ;
    run_file(input);
}
struct NextPHP {}

fn run_file(source: String) {
    let input = fs::read_to_string(source)
        .expect("expected file");
    run(input);
}
fn run(input: String) {
    println!("{}", input[1]);
    let mut scanner =Scanner::new(input[1]);
    let tokens =  scanner.scan_tokens();
    for Token in tokens {
        println!("{:?}", Token);
    };
}
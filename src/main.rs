mod lexer;
mod parser;
mod translate;

use std::io;

use crate::translate::translate;

fn main() {
    let mut source: String = String::new();

    io::stdin()
        .read_line(&mut source)
        .expect("Failed to read user input.");

    let tokens: Vec<lexer::Token> = lexer::lexer(&source);

    println!("Tokens: [ \n  {:?} \n ]", tokens);

    let ast = parser::parse(tokens);
    println!("AST: [ \n  {:?} \n ]", ast);

    let translated = translate(ast);
    println!("Translated: \n{}", translated);
}

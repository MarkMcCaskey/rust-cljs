#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

pub mod errors;
pub mod token;
pub mod lex;

fn main() {
    let input = r"
( #{ [ {} () & ])

";
    let mut lexer = lex::LexContext::create(input);

    lexer.lex();

    println!("{:?}", lexer);
}

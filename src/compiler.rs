use self::{lexer::lex, parser::parse};

#[path = "lexer.rs"]
mod lexer;
#[path = "parser.rs"]
mod parser;

pub fn compile(code: String) -> Vec<u8> {
    emit(parse(lex(code)))
}

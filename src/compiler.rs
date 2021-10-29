use self::{emitter::emit, lexer::lex, parser::parse};

#[path = "emitter.rs"]
mod emitter;
#[path = "lexer.rs"]
mod lexer;
#[path = "parser.rs"]
mod parser;

pub fn compile(code: String) -> Vec<u8> {
    emit(parse(lex(code)))
}

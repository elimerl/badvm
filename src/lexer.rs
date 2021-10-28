#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
    Int,
    Return,
    Identifier(String),
    Integer(i32),
    End, // Plus,
         // Minus,
         // Star,
         // Slash,
         // Bang
}

pub fn lex(code: String) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut read_position: usize = 0;
    while read_position < code.len() {
        let c = code.chars().nth(read_position).unwrap();
        if c.is_whitespace() {
            read_position += 1;
            continue;
        }
        match c {
            '{' => {
                tokens.push(Token::OpenBrace);
            }
            '}' => {
                tokens.push(Token::CloseBrace);
            }
            '(' => {
                tokens.push(Token::OpenParen);
            }
            ')' => {
                tokens.push(Token::CloseParen);
            }
            ';' => {
                tokens.push(Token::Semicolon);
            }
            '0'..='9' => {
                let mut value = String::new();
                value.push(c);
                while read_position < code.len() {
                    let c = code.chars().nth(read_position).unwrap();
                    if c.is_digit(10) {
                        value.push(c);
                        read_position += 1;
                    } else {
                        break;
                    }
                }
                read_position -= 1;
                tokens.push(Token::Integer(value.parse().unwrap()));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut value = String::new();
                while read_position < code.len() {
                    let c = code.chars().nth(read_position).unwrap();
                    if c.is_alphanumeric() || c == '_' {
                        value.push(c);
                        read_position += 1;
                    } else {
                        break;
                    }
                }
                read_position -= 1;
                match value.as_str() {
                    "return" => tokens.push(Token::Return),
                    "int" => tokens.push(Token::Int),
                    _ => tokens.push(Token::Identifier(value)),
                }
            }
            _ => panic!("Invalid token {}", c),
        }
        read_position += 1;
    }
    tokens.push(Token::End);
    tokens
}

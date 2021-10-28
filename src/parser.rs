use super::lexer::Token;
#[derive(Debug)]
pub enum ASTNode {
    FunDecl(String, Vec<Statement>),
    Prog(Box<crate::compiler::parser::ASTNode>),
}
#[derive(Debug)]
pub enum Statement {
    Exp(Expression),
    Return(Box<Expression>),
}
#[derive(Debug)]
pub enum Expression {
    Num(i32),
    // Add(Box<Expression>, Box<Expression>),
    // Sub(Box<Expression>, Box<Expression>),
    // Mul(Box<Expression>, Box<Expression>),
    // Div(Box<Expression>, Box<Expression>),
}
pub fn parse(tokens: Vec<Token>) -> ASTNode {
    let mut tokens = tokens;
    let mut ast = ASTNode::Prog(Box::new(ASTNode::FunDecl("main".to_string(), vec![])));
    while tokens.len() > 0 {
        match tokens.remove(0) {
            Token::Int => {
                let name = match tokens.remove(0) {
                    Token::Identifier(name) => name,
                    _ => panic!("Expected identifier"),
                };
                assert_eq!(tokens.remove(0), Token::OpenParen);
                assert_eq!(tokens.remove(0), Token::CloseParen);

                let mut statements = vec![];
                assert_eq!(tokens.remove(0), Token::OpenBrace);
                while tokens.len() > 0 {
                    match tokens.remove(0) {
                        Token::Return => match tokens.remove(0) {
                            Token::Integer(num) => {
                                statements.push(Statement::Return(Box::new(Expression::Num(num))));
                                assert_eq!(tokens.remove(0), Token::Semicolon);
                                break;
                            }
                            _ => {
                                panic!("Unexpected token");
                            }
                        },
                        Token::CloseBrace => {
                            break;
                        }
                        _ => {
                            panic!("Unexpected token");
                        }
                    }
                }
                assert_eq!(tokens.remove(0), Token::CloseBrace);
                ast = ASTNode::Prog(Box::new(ASTNode::FunDecl(name, statements)));
            }
            Token::End => {
                return ast;
            }
            _ => {
                panic!("Unexpected token");
            }
        }
    }
    ast
}

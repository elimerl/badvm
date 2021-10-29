use super::lexer::Token;
use crate::Instruction;
use std::collections::HashMap;
#[derive(Debug)]
pub enum ASTNode {
    FunDecl(String, Vec<Statement>),
    Prog(Box<crate::compiler::parser::ASTNode>),
}
struct InstrValuePair {
    instr: Instruction,
    value: Option<i64>,
    fn_name: Option<String>,
}
impl InstrValuePair {
    fn process(&mut self, fn_offsets: &HashMap<String, usize>) {
        if let Some(fn_name) = &self.fn_name {
            if let Some(offset) = fn_offsets.get(fn_name) {
                self.value = Some(*offset as i64);
            }
        }
    }
    fn to_bytes(&mut self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.push(self.instr as u8);
        if let Some(value) = self.value {
            vec.append(&mut value.to_le_bytes().to_vec());
        }
        if let Some(_fn_name) = &self.fn_name {
            vec.append(&mut vec![0; 8]);
        }
        vec
    }
}
impl ASTNode {
    pub fn emit(&self, instrs: &mut Vec<u8>) {
        let mut functions = HashMap::new();
        match self {
            ASTNode::FunDecl(name, stmts) => {
                let mut pair = Vec::<InstrValuePair>::new();
                for stmt in stmts {
                    stmt.visit(&mut pair);
                }
                functions.insert(name.clone(), pair);
            }
            &ASTNode::Prog(ref node) => {
                node.emit(instrs);
            }
        }
        let mut fn_offsets = HashMap::new();
        let mut offset: usize = 0;
        for (name, pairs) in functions {
            for mut pair in pairs {
                pair.process(&fn_offsets);
                instrs.append(&mut pair.to_bytes());
            }
            fn_offsets.insert(name, offset);
            offset += instrs.len();
        }
    }
}
#[derive(Debug)]
pub enum Statement {
    Exp(Expression),
    Return(Box<Expression>),
}
impl Statement {
    fn visit(&self, pairs: &mut Vec<InstrValuePair>) {
        match self {
            Statement::Exp(exp) => {
                exp.visit(pairs);
            }
            Statement::Return(exp) => {
                exp.visit(pairs);
                pairs.push(InstrValuePair {
                    instr: Instruction::Ret,
                    value: None,
                    fn_name: None,
                });
            }
        }
    }
}
#[derive(Debug)]
pub enum Expression {
    Num(i64),
    // Add(Box<Expression>, Box<Expression>),
    // Sub(Box<Expression>, Box<Expression>),
    // Mul(Box<Expression>, Box<Expression>),
    // Div(Box<Expression>, Box<Expression>),
}
impl Expression {
    fn visit(&self, pairs: &mut Vec<InstrValuePair>) {
        match self {
            Expression::Num(num) => {
                pairs.push(InstrValuePair {
                    instr: Instruction::Push,
                    value: Some(*num),
                    fn_name: None,
                });
            } // Expression::Add(lhs, rhs) => {
              //     lhs.visit(pairs);
              //     rhs.visit(pairs);
              //     pairs.push(InstrValuePair {
              //         instr: Instruction::Add,
              //         value: None,
              //     });
              // }
              // Expression::Sub(lhs, rhs) => {
              //     lhs.visit(pairs);
              //     rhs.visit(pairs);
              //     pairs.push(InstrValuePair {
              //         instr: Instruction::Sub,
              //         value: None,
              //     });
              // }
              // Expression::Mul(lhs, rhs) => {
              //     lhs.visit(pairs);
              //     rhs.visit(pairs);
              //     pairs.push(InstrValuePair {
              //         instr: Instruction::Mul,
              //         value: None,
              //     });
              // }
              // Expression::Div(lhs, rhs) => {
              //     lhs.visit(pairs);
              //     rhs.visit(pairs);
              //     pairs.push(InstrValuePair {
              //         instr: Instruction::Div,
              //         value: None,
              //     });
              // }
        }
    }
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
                while tokens.is_empty() {
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

use crate::lexer::{self, Token};
use std::{env::Args, mem::discriminant};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Number(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    Variable(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    VariableDeclaration { name: String, value: Statement },
    VariableAssignment { name: String, value: Statement },
    FunctionCall { name: String, args: Vec<Statement> },
    FunctionDef { name: String, args: Vec<Statement>, children: Vec<ASTNode> },
}

pub fn generate_tree(tokens: Vec<lexer::Token>) -> Vec<ASTNode> {
    let mut result: Vec<ASTNode> = Vec::new();

    let mut i = 0;
    while i < tokens.len()-1 {
        let tok = &tokens[i];
        //println!("{:?}", tok);
        
        match tok {
            Token::EndStatement => i+=1,
            Token::Var => {
                match &tokens[i+1] {
                    Token::Identifier(var_name) => {
                        match &tokens[i+2] {
                            Token::Assign => {
                                match &tokens[i+3] {
                                    Token::StringLiteral(val) => {
                                        result.push(ASTNode::VariableDeclaration { name: var_name.to_owned() , value: Statement::StringLiteral(val.to_owned()) });
                                    },
                                    Token::Number(val) => {
                                        result.push(ASTNode::VariableDeclaration { name: var_name.to_owned() , value: Statement::Number(val.to_owned()) });
                                    },
                                    Token::True => {
                                        result.push(ASTNode::VariableDeclaration { name: var_name.to_owned() , value: Statement::BoolLiteral(true) });
                                    }
                                    Token::False => {
                                        result.push(ASTNode::VariableDeclaration { name: var_name.to_owned() , value: Statement::BoolLiteral(false) });
                                    }
                                    _ => panic!("Unexpected assigned value"),
                                };
                                i += 4;
                                println!("Defined variable");
                            },
                            _ => panic!("Assignment operator expected"),
                        }
                    },
                    _ => panic!("Variable identifier expected"),
                }
            },
            Token::Fn => {
                match &tokens[i+1] {
                    Token::Identifier(name) => {
                        match &tokens[i+2] {
                            Token::LParen => {
                                let mut a = 0;
                                let mut args: Vec<Statement> = Vec::new();
                                while discriminant(&tokens[i+3+a]) != discriminant(&Token::RParen) {
                                    match &tokens[i+3+a] {
                                        Token::Identifier(name) => {
                                            args.push(Statement::Variable(name.to_owned()));
                                        },
                                        _ => panic!("Invalid arguement definition")
                                    }
                                    a += 1
                                }
                                //result.push(ASTNode::FunctionDef { name: name.to_owned(), args: args, children: });
                                println!("Function definition");
                                i += 3+a;
                            }
                            _ => panic!("No left parentheses")
                        }
                    }
                    _ => panic!("Function identifier expected.")
                }
            },
            Token::Identifier(name) => {
                match &tokens[i+1] {
                    Token::LParen => {
                        let mut a = 0;
                        let mut args: Vec<Statement> = Vec::new();
                        while discriminant(&tokens[i+2+a]) != discriminant(&Token::RParen) {
                            match &tokens[i+2+a] {
                                Token::StringLiteral(val) => {
                                    args.push(Statement::StringLiteral(val.to_owned()));
                                },
                                Token::Number(val) => {
                                    args.push(Statement::Number(val.to_owned()));
                                },
                                Token::True => {
                                    args.push(Statement::BoolLiteral(true));
                                }
                                Token::False => {
                                    args.push(Statement::BoolLiteral(false));
                                },
                                Token::Identifier(name) => {
                                    args.push(Statement::Variable(name.to_owned()));
                                }
                                Token::Comma => {},
                                _ => panic!("Invalid arguement definition")
                            }
                            a += 1
                        }
                        i += 3+a;
                        result.push(ASTNode::FunctionCall { name: name.to_owned(), args: args});
                        println!("Function call");
                    },
                    Token::Assign => {
                        match &tokens[i+2] {
                            Token::Number(val) => {
                                result.push(ASTNode::VariableAssignment { name: name.to_owned(), value: Statement::Number(val.to_owned()) });
                            },
                            Token::StringLiteral(val) => {
                                result.push(ASTNode::VariableAssignment { name: name.to_owned(), value: Statement::StringLiteral(val.to_owned()) });
                            },
                            Token::True => {
                                result.push(ASTNode::VariableAssignment { name: name.to_owned(), value: Statement::BoolLiteral(true) });
                            }
                            Token::False => {
                                result.push(ASTNode::VariableAssignment { name: name.to_owned(), value: Statement::BoolLiteral(false) });
                            }
                            _ => panic!("Cannot assign a variable to that.")
                        }
                        i += 3;
                        println!("Assigning to a variable");
                    }
                    _ => panic!("Unexpected identifier use")
                }
            },
            _ => {
                panic!("Issue here");
            }
        }
    }

    return result;
}
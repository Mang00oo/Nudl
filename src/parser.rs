use crate::lexer::{self, Token};

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    VariableDeclaration { name: String, value: Vec<Token> },
    VariableAssignment { name: String, value: Vec<Token> },
    FunctionCall { name: String, args: Vec<Token> },
    FunctionDef { name: String, args: Vec<Token>, children: Vec<ASTNode> },
    IfStatement { conditions: Vec<Token>, children: Vec<ASTNode> },
}
pub fn get_operator(tokens: &Vec<lexer::Token>, start_index: usize) -> (Vec<Token>, usize) {
    let mut tks: Vec<Token> = Vec::new();
    let mut i = start_index;
    while i < tokens.len() && tokens[i] != Token::LBlock && tokens[i] != Token::RParen && tokens[i] != Token::EndStatement {
        tks.push(tokens[i].to_owned());
        i += 1;
    }
    return (tks, i);
}

pub fn generate_tree(tokens: &Vec<lexer::Token>, start_index: usize) -> (Vec<ASTNode>, usize) {
    let mut result: Vec<ASTNode> = Vec::new();

    let mut i = start_index;
    while i < tokens.len()-1 && tokens[i] != Token::RBlock {
        let tok = &tokens[i];
        //println!("{:?}", tok);
        
        match tok {
            Token::EndStatement => i+=1,
            Token::Var => {
                match &tokens[i+1] {
                    Token::Identifier(var_name) => {
                        match &tokens[i+2] {
                            Token::Assign => {
                                let operator = get_operator(&tokens, i+3);
                                result.push(ASTNode::VariableDeclaration { name: var_name.to_owned(), value: operator.0 });
                                i = operator.1;
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
                                let operator = get_operator(&tokens, i+3);
                                println!("Function definition");
                                i = operator.1;
                                match &tokens[i+1] {
                                    Token::LBlock => {
                                        let children = generate_tree(&tokens, i+2);
                                        result.push(ASTNode::FunctionDef { name: name.to_owned(), args: operator.0, children: children.0});
                                        i = children.1+1;
                                        println!("Thing 2: {:?}", &tokens[i]);
                                    }
                                    _ => {
                                        panic!("Expected left block, found: {:?}", &tokens[i+1]);
                                    }
                                }
                            }
                            _ => panic!("No left parentheses")
                        }
                    }
                    _ => panic!("Function identifier expected.")
                }
            },
            Token::If => {
                let operator = get_operator(&tokens, i+1);
                match &tokens[operator.1] {
                    Token::LBlock => {
                        let children = generate_tree(&tokens, operator.1+1);
                        result.push(ASTNode::IfStatement { conditions: operator.0, children: children.0 });
                        println!("{:?}", result[result.len()-1]);
                        i = children.1+1;
                        println!("{:?}", &tokens[children.1]);
                    }
                    _ => panic!("LBlock expected: {:?}", &tokens[operator.1])
                }
            }
            Token::Identifier(name) => {
                match &tokens[i+1] {
                    Token::LParen => {
                        let mut args = get_operator(&tokens, i+2);
                        let res = ASTNode::FunctionCall { name: name.to_owned(), args: args.0};
                        result.push(res.to_owned());
                        i = args.1+2;
                        println!("Function call: {:?}", &res);
                    },
                    Token::Assign => {
                        let operator = get_operator(&tokens, i+2);
                        i = operator.1;
                        result.push(ASTNode::VariableAssignment { name: name.to_owned(), value: operator.0 });
                        println!("Assigning to a variable");
                    }
                    _ => panic!("Unexpected identifier use")
                }
            },
            _ => {
                panic!("Issue here: {:?}", &tokens[i]);
            },
        }
    }
    return (result, i);
}
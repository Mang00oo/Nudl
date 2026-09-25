use core::panic;
use line_index::LineIndex;
use colored::Colorize;
use std::collections::HashMap;

use crate::lexer::{self, Token};

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    VariableDeclaration { name: String, value: Vec<Token> },
    VariableAssignment { name: String, value: Vec<Token> },
    FunctionCall { name: String, args: Vec<Token> },
    FunctionDef { name: String, args: Vec<Token>, children: Vec<ASTNode>, returns: VarType },
    FunctionReturn { args: Vec<Token> },
    IfStatement { conditions: Vec<Token>, children: Vec<ASTNode> },
    ElseStatement { children: Vec<ASTNode> },
    ElseIfStatement { conditions: Vec<Token>, children: Vec<ASTNode> },
    WhileStatement { conditions: Vec<Token>, children: Vec<ASTNode> },
    ForStatement { identifier: String, conditions: Vec<Token>, children: Vec<ASTNode> },
    LoopBreak,
    LoopContinue,
}
#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    String,
    Number,
    Boolean,
    None,
}
pub fn get_operator(tokens: &Vec<(lexer::Token, lexer::Span)>, start_index: usize) -> (Vec<Token>, usize) {
    let mut tks: Vec<Token> = Vec::new();
    let mut i = start_index;
    let mut paren_depth = 0;
    while i < tokens.len() && tokens[i].0 != Token::LBlock && paren_depth >= 0 && tokens[i].0 != Token::EndStatement {
        match &tokens[i].0 {
            Token::LParen => {paren_depth += 1},
            Token::RParen => {paren_depth -= 1;
                if paren_depth < 0 { break }
            }
            _ => {}
        }
        tks.push(tokens[i].0.to_owned());
        i += 1;
    }
    return (tks, i);
}

pub fn generate_error_message(span: &lexer::Span, message: &str, source: &str) {
    let line_start = &source[..span.lo].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let line_end = &source[span.lo..].find('\n').map(|i| span.lo + i).unwrap_or(source.len());
    let full_line = &source[line_start.to_owned()..line_end.to_owned()];
    let issue = &source[span.lo..span.hi];
    let line_index = LineIndex::new(source);
    let coord = line_index.line_col(8.into());

    let mut arrows = String::new();
    for i in 0..span.lo-line_start { arrows.push_str(" "); }
    for i in 0..(span.hi-span.lo) { arrows.push_str("^"); }
    println!("");
    println!("{}", "Error translating!".red());
    println!("{}", full_line);
    println!("{}", arrows.red());
    println!(" -> {}, found {} (line {}, col {})", message, issue, coord.line, coord.col);
    panic!(" ");
}
fn infer_type (tokens: &Vec<Token>) -> VarType {
    if tokens.contains(&lexer::Token::Greater) { return VarType::Boolean };
    if tokens.contains(&lexer::Token::EGreater) { return VarType::Boolean };
    if tokens.contains(&lexer::Token::Less) { return VarType::Boolean };
    if tokens.contains(&lexer::Token::ELess) { return VarType::Boolean };
    if tokens.contains(&lexer::Token::Plus) { return VarType::Number };
    if tokens.contains(&lexer::Token::Minus) { return VarType::Number };
    if tokens.contains(&lexer::Token::Multiply) { return VarType::Number };
    if tokens.contains(&lexer::Token::Divide) { return VarType::Number };
    if tokens.contains(&lexer::Token::Mod) { return VarType::Number };
    println!("Tokens: {:?}", &tokens);
    match &tokens[0] {
        lexer::Token::StringLiteral(val) => return VarType::String,
        lexer::Token::Number(val) => return VarType::Number,
        lexer::Token::True => return VarType::Boolean,
        lexer::Token::False => return VarType::Boolean,
        _ => panic!("Could not infer type for: {:?}", &tokens[0])
    }
}

pub fn generate_tree(tokens: &Vec<(lexer::Token, lexer::Span)>, start_index: usize, source: &str) -> (Vec<ASTNode>, usize) {
    let mut result: Vec<ASTNode> = Vec::new();

    let mut var_types: HashMap<&String, VarType> = HashMap::new();

    let mut i = start_index;
    while i < tokens.len()-1 && tokens[i].0 != Token::RBlock {
        let tok = &tokens[i].0;
        //println!("{:?}", &tokens[i].1);
        
        match tok {
            Token::EndStatement => i+=1,
            Token::Var => {
                match &tokens[i+1].0 {
                    Token::Identifier(var_name) => {
                        match &tokens[i+2].0 {
                            Token::Assign => {
                                let operator = get_operator(&tokens, i+3);
                                result.push(ASTNode::VariableDeclaration { name: var_name.to_owned(), value: operator.0 });
                                i = operator.1;
                            },
                            _ => generate_error_message(&tokens[i+2].1, "Assignment operator expected", source),
                        }
                    },
                    _ => generate_error_message(&tokens[i+1].1, "Variable identifier expected", source),
                }
            },
            Token::Fn => {
                match &tokens[i+1].0 {
                    Token::Identifier(name) => {
                        match &tokens[i+2].0 {
                            Token::LParen => {
                                let operator = get_operator(&tokens, i+3);
                                i = operator.1;
                                match &tokens[i+1].0 {
                                    Token::LBlock => {
                                        let children = generate_tree(&tokens, i+2, source);
                                        let mut ty = VarType::None;
                                        for node in &children.0 {
                                            match node {
                                                ASTNode::FunctionReturn { args } => {
                                                    ty = infer_type(args);
                                                },
                                                _ => continue
                                            }
                                        }
                                        result.push(ASTNode::FunctionDef { name: name.to_owned(), args: operator.0, children: children.0, returns: ty});
                                        i = children.1+1;
                                    }
                                    _ => generate_error_message(&tokens[i+1].1, "Expected left block", source),
                                }
                            }
                            _ => generate_error_message(&tokens[i+2].1, "No left parentheses", source)
                        }
                    }
                    _ => generate_error_message(&tokens[i+1].1, "Function identifier expected", source),
                }
            },
            Token::If => {
                let operator = get_operator(&tokens, i+1);
                match &tokens[operator.1].0 {
                    Token::LBlock => {
                        let children = generate_tree(&tokens, operator.1+1, source);
                        result.push(ASTNode::IfStatement { conditions: operator.0, children: children.0 });
                        i = children.1+1;
                    }
                    _ => generate_error_message(&tokens[operator.1].1, "Left block expected", source),
                }
            }
            Token::Return => {
                let operator = get_operator(&tokens, i+1);
                result.push(ASTNode::FunctionReturn { args: operator.0 });
                i = operator.1+1;
            }
            Token::While => {
                let operator = get_operator(&tokens, i+1);
                match &tokens[operator.1].0 {
                    Token::LBlock => {
                        let children = generate_tree(&tokens, operator.1+1, source);
                        result.push(ASTNode::WhileStatement { conditions: operator.0, children: children.0 });
                        i = children.1+1;
                    }
                    _ => generate_error_message(&tokens[operator.1].1, "Left block expected", source),
                }
            }
            Token::For => {
                match &tokens[i+1].0 {
                    Token::Identifier(name) => {
                        match &tokens[i+2].0 {
                            Token::In => {
                                let operator = get_operator(&tokens, i+3);
                                let children = generate_tree(&tokens, operator.1+1, source);
                                result.push(ASTNode::ForStatement { identifier: name.to_owned(), conditions: operator.0, children: children.0 });
                                i = children.1+1;
                            },
                            _ => generate_error_message(&tokens[i+1].1, "Identifier expected", source),
                        };
                    }
                    _ => generate_error_message(&tokens[i+1].1, "Identifier expected", source),
                }
            }
            Token::Else => {
                match &result[result.len()-1] {
                    ASTNode::IfStatement { conditions, children} => {},
                    ASTNode::ElseIfStatement { conditions, children} => {},
                    _ =>  generate_error_message(&tokens[i].1, "Preceeding if or else if expected", source),
                }
                match &tokens[i+1].0 {
                    Token::LBlock => {
                        let children = generate_tree(&tokens, i+2, source);
                        result.push(ASTNode::ElseStatement { children: children.0 });
                        i = children.1+1;
                    },
                    Token::If => {
                        let operator = get_operator(&tokens, i+2);
                        match &tokens[operator.1].0 {
                            Token::LBlock => {
                                let children = generate_tree(&tokens, operator.1+1, source);
                                result.push(ASTNode::ElseIfStatement { conditions: operator.0, children: children.0 });
                                i = children.1+1;
                            }
                            _ => generate_error_message(&tokens[operator.1].1, "Left block expected", source),
                        }
                    }
                    _ => generate_error_message(&tokens[i+1].1, "Left block expected", source),
                }
            }
            Token::Identifier(name) => {
                match &tokens[i+1].0 {
                    Token::LParen => {
                        let mut args = get_operator(&tokens, i+2);
                        let res = ASTNode::FunctionCall { name: name.to_owned(), args: args.0};
                        result.push(res.to_owned());
                        i = args.1+2;
                    },
                    Token::Assign => {
                        let operator = get_operator(&tokens, i+2);
                        i = operator.1;
                        result.push(ASTNode::VariableAssignment { name: name.to_owned(), value: operator.0 });
                    }
                    _ => generate_error_message(&tokens[i+1].1, "Unexpected identifier usage", source),
                }
            },
            Token::StartComment => {
                let mut j = i;
                while &tokens[j].0 != &Token::EndComment {
                    j += 1;
                }
                i = j+1;
            },
            Token::Break => {
                result.push(ASTNode::LoopBreak);
                i += 1;
            }
            Token::Continue => {
                result.push(ASTNode::LoopContinue);
                i += 1;
            }
            _ => generate_error_message(&tokens[i].1, "Unknown issue", source),
        }
    }
    return (result, i);
}
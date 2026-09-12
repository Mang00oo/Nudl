use std::fs;
use std::process::Command;
use std::env;
use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::lexer;
use crate::parser;

#[derive(Serialize, Deserialize)]
pub struct Flavor {
    pub statically_typed: bool,

    pub block_def: String,
    pub parentheses: String,
    pub comments: String,
    pub line_end: String,
    pub dot: String,
    pub comma: String,
    pub assignment_op: String,
    pub string_literal: String,
    pub true_literal: String,
    pub false_literal: String,

    pub add_op: String,
    pub sub_op: String,
    pub mult_op: String,
    pub div_op: String,

    pub equal_op: String,
    pub not_equal_op: String,
    pub greater_op: String,
    pub less_op: String,
    pub eo_greater_op: String,
    pub eo_less_op: String,

    pub function_def: String,
    pub variable_def: String,

    pub if_statement: String,
    pub else_statement: String,
    pub for_statement: String,
    pub for_in: String,
    pub while_statement: String,
}
fn operation_to_string(operation: Vec<lexer::Token>) -> String {
    let mut op_str = String::new();
    for (i, token) in operation.iter().enumerate() {
        match token {
            lexer::Token::Plus => op_str.push_str("+"),
            lexer::Token::Minus => op_str.push_str("-"),
            lexer::Token::Multiply => op_str.push_str("*"),
            lexer::Token::Divide => op_str.push_str("/"),
            lexer::Token::LParen => op_str.push_str("("),
            lexer::Token::RParen => op_str.push_str(")"),
            lexer::Token::True => op_str.push_str("true"),
            lexer::Token::False => op_str.push_str("false"),
            lexer::Token::Comma => op_str.push_str(","),
            lexer::Token::Equal => op_str.push_str("=="),
            lexer::Token::NotEqual => op_str.push_str("!="),
            lexer::Token::Greater => op_str.push_str(">"),
            lexer::Token::EGreater => op_str.push_str(">="),
            lexer::Token::Less => op_str.push_str("<"),
            lexer::Token::ELess => op_str.push_str("<="),
            lexer::Token::Dot => {op_str.pop(); op_str.push_str(".");},
            lexer::Token::Identifier(name) => op_str.push_str(&name),
            lexer::Token::StringLiteral(val) => op_str.push_str(format!("\"{}\"", val).as_str()),
            lexer::Token::Number(val) => op_str.push_str(&val.to_string()),
            _ => panic!("Invalid operand: {:?}", token)
        }
        if i < operation.len()-1 {
            op_str.push_str(" ");
        }
    }
    return op_str;
}
fn get_indented_string(numIndents: i32) -> String {
    let mut indented = String::new();
    for b in 0..numIndents {
        indented.push_str("\t");
    }
    return indented;
}

fn generate_translated(tree: &Vec<parser::ASTNode>, block_depth: i32) -> (String, usize) {
    let mut translated = String::new();

    let mut i = 0;
    while i < tree.len() {
        translated.push_str(&get_indented_string(block_depth));
        match &tree[i] {
            parser::ASTNode::FunctionCall { name, args } => {
                if name == "print" {
                    translated.push_str(&format!("println!({});", operation_to_string(args.to_owned())));
                } else if name == "range" {
                    translated.push_str(&format!("0..{}", operation_to_string(args.to_owned())));
                } else {
                    translated.push_str(&format!("{}({});", name, operation_to_string(args.to_owned())));
                }
            }
            parser::ASTNode::VariableDeclaration { name, value } => {
                translated.push_str(&format!("let mut {} = {};", name, operation_to_string(value.to_owned())));
            },
            parser::ASTNode::VariableAssignment { name, value } => {
                translated.push_str(&format!("{} = {};", name, operation_to_string(value.to_owned())));
            },
            parser::ASTNode::FunctionDef { name, args, children } => {
                translated.push_str(&format!("let {} = |{}| {{ \n", name.to_owned(), operation_to_string(args.to_owned())));
                translated.push_str(&generate_translated(children, block_depth+1).0);
                translated.push_str(&get_indented_string(block_depth));
                translated.push_str("};");
            },
            parser::ASTNode::IfStatement { conditions, children } => {
                translated.push_str(&format!("if {} {{ \n", operation_to_string(conditions.to_owned())));
                translated.push_str(&generate_translated(children, block_depth+1).0);
                translated.push_str(&get_indented_string(block_depth));
                translated.push_str("}");
            }
            parser::ASTNode::ElseStatement { children } => {
                match &tree[i-1] {
                    parser::ASTNode::IfStatement { conditions, children} => {},
                     parser::ASTNode::ElseIfStatement { conditions, children} => {},
                    _ => panic!("Expected if or else if statement before")
                }
                for i in 0..block_depth+1 {
                    translated.pop();
                }
                translated.push_str(&format!(" else {{ \n"));
                translated.push_str(&generate_translated(children, block_depth+1).0);
                translated.push_str(&get_indented_string(block_depth));
                translated.push_str("}");
            }
            parser::ASTNode::ElseIfStatement { conditions, children } => {
                match &tree[i-1] {
                    parser::ASTNode::IfStatement { conditions, children} => {},
                    parser::ASTNode::ElseIfStatement { conditions, children} => {},
                    _ => panic!("Expected if or else if statement before")
                }
                for i in 0..block_depth+1 {
                    translated.pop();
                }
                translated.push_str(&format!(" else if {} {{ \n", operation_to_string(conditions.to_owned())));
                translated.push_str(&generate_translated(children, block_depth+1).0);
                translated.push_str(&get_indented_string(block_depth));
                translated.push_str("}");
            },
            parser::ASTNode::WhileStatement { conditions, children } => {
                translated.push_str(&format!("while {} {{ \n", operation_to_string(conditions.to_owned())));
                translated.push_str(&generate_translated(children, block_depth+1).0);
                translated.push_str(&get_indented_string(block_depth));
                translated.push_str("}");
            },
            parser::ASTNode::ForStatement { identifier, conditions, children } => {
                translated.push_str(&format!("for {} in {} {{ \n", identifier, operation_to_string(conditions.to_owned())));
                translated.push_str(&generate_translated(children, block_depth+1).0);
                translated.push_str(&get_indented_string(block_depth));
                translated.push_str("}");
            }
            _ => {}
        }
        translated.push_str("\n");
        i += 1;
    }
    return (translated, i);
}

fn write_rust_file(tree: Vec<parser::ASTNode>) {
    let mut translated = String::new();

    translated.push_str("fn main() { \n");

    translated.push_str(&generate_translated(&tree, 1).0);

    translated.push_str("}");

    fs::write("translated/src/main.rs", translated).expect("Couldn't write Rust file.");
}

pub fn translate() {
    let main_file = fs::read_to_string("src/main.nudl").expect("No main script found.");
    let flavor_file = fs::read_to_string("flavor.json").expect("No flavor file found.");
    let flavor: Flavor = serde_json::from_str(&flavor_file).expect("Couldn't parse flavor file.");
    for entry in fs::read_dir("maps").expect("Couldn't read maps") {
        let entry = entry.expect("Couldn't read map file.");
        let file_path = entry.path();
        let mapContent = fs::read_to_string(file_path);
    }
    println!("Tokenizing...");
    let tokens = lexer::tokenize(&main_file, flavor);
    println!("Parsing...");
    let tree = parser::generate_tree(&tokens, 0, &main_file);
    write_rust_file(tree.0);
    let mut current_dir = env::current_dir().expect("Failed to get current directory").join("translated");
    let rustOutput = Command::new("cargo")
            .current_dir(&current_dir)
            .arg("fix")
            .arg("--bin")
            .arg(format!("{}", "my-project"))
            .arg("-p")
            .arg("my-project")
            .arg("--allow-dirty")
            .status()
            .expect("Couldn't build Rust project.");
}
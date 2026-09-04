use std::fs;
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
    pub while_statement: String,
}
fn statement_to_string(statement: parser::Statement) -> String {
    match statement {
        parser::Statement::BoolLiteral(val) => {
            return format!("{:?}", val);
        },
        parser::Statement::StringLiteral(val) => {
            return format!("{:?}", val);
        },
        parser::Statement::Number(val) => {
            return format!("{:?}", val);
        },
        parser::Statement::Variable(name) => {
            return format!("{}", name);
        }
    }
}

fn args_to_string(statements: Vec<parser::Statement>) -> String {
    let mut new = String::new();

    for (i, statement) in statements.iter().enumerate() {
        new.push_str(&statement_to_string(statement.to_owned()));
        if i < statements.len() - 1 {
            new.push_str(", ");
        }
    }
    return new;
}

fn write_rust_file(tree: Vec<parser::ASTNode>) {
    let mut translated = String::new();

    translated.push_str("fn main() { \n");

    let mut block_depth = 1;

    for node in tree {
        for i in [0..block_depth] {
            translated.push_str("\t");
        }
        match node {
            parser::ASTNode::FunctionCall { name, args } => {
                if (name == "print") {
                    translated.push_str(&format!("println!({});", args_to_string(args)));
                }
            }
            parser::ASTNode::VariableDeclaration { name, value } => {
                translated.push_str(&format!("let mut {} = {};", name, statement_to_string(value)));
            },
            parser::ASTNode::VariableAssignment { name, value } => {
                translated.push_str(&format!("{} = {};", name, statement_to_string(value)));
            },
            _ => {}
        }
        translated.push_str("\n");
    }

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
    let tree = parser::generate_tree(tokens);
    write_rust_file(tree);
}
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::lexer;

#[derive(Serialize, Deserialize)]
pub struct Flavor {
    pub statically_typed: bool,

    pub block_def: String,
    pub parentheses: String,
    pub comments: String,
    pub line_end: String,
    pub dot: String,
    pub assignment_op: String,
    pub string_literal: String,

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

pub fn translate() {
    let mainFile = fs::read_to_string("src/main.nudl").expect("No main script found.");
    let flavorFile = fs::read_to_string("flavor.json").expect("No flavor file found.");
    let flavor: Flavor = serde_json::from_str(&flavorFile).expect("Couldn't parse flavor file.");
    for entry in fs::read_dir("maps").expect("Couldn't read maps") {
        let entry = entry.expect("Couldn't read map file.");
        let file_path = entry.path();
        let mapContent = fs::read_to_string(file_path);
    }
    lexer::tokenize(&mainFile, flavor);
}
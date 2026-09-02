use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Flavor {
    statically_typed: bool,

    block_def: String,
    comments: String,
    line_end: String,

    function_def: String,
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
}
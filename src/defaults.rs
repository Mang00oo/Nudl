pub fn flavorFileDefault() -> &'static str {
    return r##"{
    "statically_typed": false,

    "block_def": "{}",
    "comments": "#",
    "line_end": ";",

    "function_def": "def"
}"##
}
pub fn stdMapDefault() -> &'static str {
    return r##"{
    "print({x})": "println!({x})"
}"##
}
pub fn mainScriptDefault() -> &'static str {
    return r##"print("Hello, world!");"##
}
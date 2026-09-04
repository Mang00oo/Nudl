pub fn flavor_file_default() -> &'static str {
    return r##"{
    "statically_typed": false,

    "block_def": "{}",
    "parentheses": "()",
    "comments": "#",
    "line_end": ";",
    "dot": ".",
    "comma": ",",
    "assignment_op": "=",
    "string_literal": "'",
    "true_literal": "true",
    "false_literal": "false",

    "add_op": "+",
    "sub_op": "-",
    "mult_op": "*",
    "div_op": "/",

    "equal_op": "==",
    "not_equal_op": "!=",
    "greater_op": ">",
    "less_op": "<",
    "eo_greater_op": ">=",
    "eo_less_op": "<=",

    "function_def": "def",
    "variable_def": "var",

    "if_statement": "if",
    "else_statement": "else",
    "for_statement": "for",
    "while_statement": "while"
}"##
}
pub fn std_map_default() -> &'static str {
    return r##"{
    "print({x})": "println!({x})"
}"##
}
pub fn main_script_default() -> &'static str {
    return r##"var x = 10;
    print("Hello, world!");
    "##
}
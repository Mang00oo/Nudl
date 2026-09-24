pub fn flavor_file_default() -> &'static str {
    return r##"{
    "block_def": "{}",
    "parentheses": "()",
    "start_comment": "#",
    "end_comment": "\n",
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

    "and_statement": "and",
    "or_statement": "or",
    "not_statement": "not",

    "function_def": "def",
    "function_return": "return",
    "variable_def": "var",

    "if_statement": "if",
    "else_statement": "else",
    "for_statement": "for",
    "for_in": "in",
    "for_iter": "range",
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
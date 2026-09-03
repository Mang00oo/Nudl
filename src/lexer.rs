use crate::translator::Flavor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(f64),
    StringLiteral(String),

    //Keywords
    Var,
    Fn,
    If,
    Else,
    While,
    Return,
    True,
    False,

    // Operators
    Assign, // =
    Plus, // +
    Minus, // -
    Multiply, // *
    Divide, // /
    Equal, // ==
    NotEqual, // !=
    EndStatement, // ;
    Comma, // ,
    Dot, // .
    LParen, RParen, // ()
    LBlock, RBlock, // {}
    Quote, // "

    EOF,

}

pub fn tokenize(source: &String, flavor: Flavor) {
    let mut result: Vec<Token> = Vec::new();

    let mut _current_token = String::new();

    
    let eval_current = |current_token: &mut String, result: &mut Vec<Token>| {
        let token = std::mem::take(current_token);
        //println!("{}", token);
        
        if token.as_str() == flavor.variable_def {
            result.push(Token::Var);
            println!("Variable declaration")
        } else if token.as_str() == flavor.function_def {
            result.push(Token::Fn);
            println!("Function declaration")
        } else if token.as_str().chars().next() == flavor.string_literal.chars().next() && token.as_str().chars().last() == flavor.string_literal.chars().next(){
            let mut chars = token.as_str().chars();
            chars.next();
            chars.next_back();
            result.push(Token::StringLiteral(chars.as_str().to_owned()));
            println!("String literal");
        } else {
            if token.as_str().chars().count() > 0 {
                result.push(Token::Identifier(token));
                println!("Identifier");
            }
        }
    };
    
    let mut is_in_literal: bool = false;
    for c in source.chars() {
        if c.is_whitespace() && !is_in_literal {
            eval_current(&mut _current_token, &mut result);
            continue;
        }
        if c == flavor.line_end.chars().next().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::EndStatement);
            println!("End Statement");
            continue
        }
        if c == flavor.dot.chars().next().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::Dot);
            println!("Dot");
            continue
        }
        if c == flavor.assignment_op.chars().next().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::Assign);
            println!("Assignment");
            continue
        }
        if c == flavor.parentheses.chars().next().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::LParen);
            println!("Left Parentheses");
            continue
        }
        if c == flavor.parentheses.chars().last().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::RParen);
            println!("Right Parentheses");
            continue
        }
        if c == flavor.string_literal.chars().next().expect("Flavor error") {
            is_in_literal = !is_in_literal;
        }
        _current_token.push(c);
    }
}
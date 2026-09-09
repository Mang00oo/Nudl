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
    Greater,
    Less,
    EGreater,
    ELess,
    EndStatement, // ;
    Comma, // ,
    Dot, // .
    LParen, RParen, // ()
    LBlock, RBlock, // {}
    Quote, // "

    EOF,

}

pub fn tokenize(source: &String, flavor: Flavor) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();

    let mut _current_token = String::new();

    
    let eval_current = |current_token: &mut String, result: &mut Vec<Token>| {
        let token = std::mem::take(current_token);
        //println!("{}", token);
        
        if token.as_str() == flavor.variable_def {
            result.push(Token::Var);
        } else if token.as_str() == flavor.function_def {
            result.push(Token::Fn);
        } else if token.as_str() == flavor.if_statement {
            result.push(Token::If);
        } else if token.as_str().chars().next() == flavor.string_literal.chars().next() && token.as_str().chars().last() == flavor.string_literal.chars().next(){
            let mut chars = token.as_str().chars();
            chars.next();
            chars.next_back();
            result.push(Token::StringLiteral(chars.as_str().to_owned()));
        } else if token.as_str() == flavor.add_op {
            result.push(Token::Plus);
        } else if token.as_str() == flavor.sub_op {
            result.push(Token::Minus);
        } else if token.as_str() == flavor.mult_op {
            result.push(Token::Multiply);
        } else if token.as_str() == flavor.div_op {
            result.push(Token::Divide);
        } else if token.as_str() == flavor.equal_op {
            result.push(Token::Equal);
        } else if token.as_str() == flavor.not_equal_op {
            result.push(Token::NotEqual);
        } else if token.as_str() == flavor.greater_op {
            result.push(Token::Greater);
        } else if token.as_str() == flavor.eo_greater_op {
            result.push(Token::EGreater);
        } else if token.as_str() == flavor.less_op {
            result.push(Token::Less);
        } else if token.as_str() == flavor.eo_less_op {
            result.push(Token::ELess);
        } else if token.as_str() == flavor.assignment_op {
            result.push(Token::Assign);
        } else if token.as_str() == flavor.true_literal {
            result.push(Token::True);
        } else if token.as_str() == flavor.false_literal {
            result.push(Token::False);
        } else if let Ok(number) = token.as_str().parse::<f64>() {
            result.push(Token::Number(number));
        } else {
            if token.as_str().chars().count() > 0 {
                result.push(Token::Identifier(token));
            }
        }
    };
    
    let mut is_in_literal: bool = false;
    for c in source.chars() {
        if c.is_whitespace() && !is_in_literal {
            eval_current(&mut _current_token, &mut result);
            continue;
        }
        if c == flavor.string_literal.chars().next().expect("Flavor error") {
            is_in_literal = !is_in_literal;
        }
        if is_in_literal {
            _current_token.push(c);
            continue
        }
        if c == flavor.line_end.chars().next().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::EndStatement);
            continue
        }
        if c == flavor.dot.chars().next().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::Dot);
            continue
        }
        if c == flavor.comma.chars().next().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::Comma);
            continue
        }
        if c == flavor.add_op.chars().next().expect("Flavor error") && flavor.add_op.chars().count() == 1 {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::Plus);
            continue
        }
        if c == flavor.sub_op.chars().next().expect("Flavor error") && flavor.sub_op.chars().count() == 1 {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::Minus);
            continue
        }
        if c == flavor.mult_op.chars().next().expect("Flavor error") && flavor.mult_op.chars().count() == 1 {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::Multiply);
            continue
        }
        if c == flavor.div_op.chars().next().expect("Flavor error") && flavor.div_op.chars().count() == 1 {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::Divide);
            continue
        }
        if c == flavor.assignment_op.chars().next().expect("Flavor error") && flavor.assignment_op.chars().count() == 1 {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::Assign);
            continue
        }
        if c == flavor.parentheses.chars().next().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::LParen);
            continue
        }
        if c == flavor.parentheses.chars().last().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::RParen);
            continue
        }
        if c == flavor.block_def.chars().next().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::LBlock);
            continue
        }
        if c == flavor.block_def.chars().last().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result);
            result.push(Token::RBlock);
            continue
        }
        _current_token.push(c);
    }
    return result;
}
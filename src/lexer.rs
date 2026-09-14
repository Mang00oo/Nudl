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
    For,
    In,
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

pub fn tokenize(source: &String, flavor: &Flavor) -> Vec<(Token, Span)> {
    let mut result: Vec<(Token, Span)> = Vec::new();

    let mut _current_token = String::new();

    
    let eval_current = |current_token: &mut String, result: &mut Vec<(Token, Span)>, index: usize| {
        let token = std::mem::take(current_token);
        let token_len = token.chars().count();
        let span = Span {
            lo: index.saturating_sub(token_len),
            hi: index,
        };

        if token.as_str() == flavor.variable_def {
            result.push((Token::Var, span));
        } else if token.as_str() == flavor.function_def {
            result.push((Token::Fn, span));
        } else if token.as_str() == flavor.if_statement {
            result.push((Token::If, span));
        } else if token.as_str() == flavor.else_statement {
            result.push((Token::Else, span));
        } else if token.as_str() == flavor.while_statement {
            result.push((Token::While, span));
        } else if token.as_str() == flavor.for_in {
            result.push((Token::In, span));
        } else if token.as_str() == flavor.for_statement {
            result.push((Token::For, span));
        } else if token.as_str().chars().next() == flavor.string_literal.chars().next() && token.as_str().chars().last() == flavor.string_literal.chars().next() {
            let mut chars = token.as_str().chars();
            chars.next();
            chars.next_back();
            result.push((Token::StringLiteral(chars.as_str().to_owned()), span));
        } else if token.as_str() == flavor.add_op {
            result.push((Token::Plus, span));
        } else if token.as_str() == flavor.sub_op {
            result.push((Token::Minus, span));
        } else if token.as_str() == flavor.mult_op {
            result.push((Token::Multiply, span));
        } else if token.as_str() == flavor.div_op {
            result.push((Token::Divide, span));
        } else if token.as_str() == flavor.equal_op {
            result.push((Token::Equal, span));
        } else if token.as_str() == flavor.not_equal_op {
            result.push((Token::NotEqual, span));
        } else if token.as_str() == flavor.greater_op {
            result.push((Token::Greater, span));
        } else if token.as_str() == flavor.eo_greater_op {
            result.push((Token::EGreater, span));
        } else if token.as_str() == flavor.less_op {
            result.push((Token::Less, span));
        } else if token.as_str() == flavor.eo_less_op {
            result.push((Token::ELess, span));
        } else if token.as_str() == flavor.assignment_op {
            result.push((Token::Assign, span));
        } else if token.as_str() == flavor.true_literal {
            result.push((Token::True, span));
        } else if token.as_str() == flavor.false_literal {
            result.push((Token::False, span));
        } else if let Ok(number) = token.as_str().parse::<f64>() {
            result.push((Token::Number(number), span));
        } else if token.as_str().chars().count() > 0 {
            result.push((Token::Identifier(token), span));
        }
    };
    
    let mut is_in_literal: bool = false;
    for (i, c) in source.chars().into_iter().enumerate() {
        if c.is_whitespace() && !is_in_literal {
            eval_current(&mut _current_token, &mut result, i);
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
            eval_current(&mut _current_token, &mut result, i);
            result.push((Token::EndStatement, Span {lo: i-1, hi: i}));
            continue
        }
        if c == flavor.dot.chars().next().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result, i);
            result.push((Token::Dot, Span {lo: i-1, hi: i}));
            continue
        }
        if c == flavor.comma.chars().next().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result, i);
            result.push((Token::Comma, Span {lo: i-1, hi: i}));
            continue
        }
        if c == flavor.add_op.chars().next().expect("Flavor error") && flavor.add_op.chars().count() == 1 {
            eval_current(&mut _current_token, &mut result, i);
            result.push((Token::Plus, Span {lo: i-1, hi: i}));
            continue
        }
        if c == flavor.sub_op.chars().next().expect("Flavor error") && flavor.sub_op.chars().count() == 1 {
            eval_current(&mut _current_token, &mut result, i);
            result.push((Token::Minus, Span {lo: i-1, hi: i}));
            continue
        }
        if c == flavor.mult_op.chars().next().expect("Flavor error") && flavor.mult_op.chars().count() == 1 {
            eval_current(&mut _current_token, &mut result, i);
            result.push((Token::Multiply, Span {lo: i-1, hi: i}));
            continue
        }
        if c == flavor.div_op.chars().next().expect("Flavor error") && flavor.div_op.chars().count() == 1 {
            eval_current(&mut _current_token, &mut result, i);
            result.push((Token::Divide, Span {lo: i-1, hi: i}));
            continue
        }
        if c == flavor.parentheses.chars().next().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result, i);
            result.push((Token::LParen, Span {lo: i-1, hi: i}));
            continue
        }
        if c == flavor.parentheses.chars().last().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result, i);
            result.push((Token::RParen, Span {lo: i-1, hi: i}));
            continue
        }
        if c == flavor.block_def.chars().next().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result, i);
            result.push((Token::LBlock, Span {lo: i-1, hi: i}));
            continue
        }
        if c == flavor.block_def.chars().last().expect("Flavor error") {
            eval_current(&mut _current_token, &mut result, i);
            result.push((Token::RBlock, Span {lo: i-1, hi: i}));
            continue
        }
        _current_token.push(c);
    }
    return result;
}
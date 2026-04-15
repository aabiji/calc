/*
- parse functions and their arguments
    - handle sin notation (ex: sin 1/2x = sin(1/2x)

- start thinking about evaluating the expression
*/

use std::collections::HashMap;
use std::iter::Peekable;

mod tests;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(char),
    Identifier(String),
    Builtin(String),
    OpenParen,
    CloseParen,
}

fn slice_string(value: &str, start: usize, end: usize) -> String {
    value
        .chars()
        .skip(start)
        .take(1 + end - start)
        .collect::<String>()
}

fn is_builtin(value: &str) -> (Option<&str>, Option<f64>) {
    let builtin_functions: HashMap<&str, u8> = HashMap::from([
        ("sin", 0),
        ("cos", 0),
        ("tan", 0),
        ("arcsin", 0),
        ("arccos", 0),
        ("arctan", 0),
        ("csc", 0),
        ("sec", 0),
        ("cot", 0),
        ("ln", 0),
    ]);

    let builtin_consts: HashMap<&str, f64> =
        HashMap::from([("π", std::f64::consts::PI), ("e", std::f64::consts::E)]);

    (
        builtin_functions.get(value).map(|_| value),
        builtin_consts.get(value).cloned(),
    )
}

fn insert_implied_tokens(tokens: &Vec<Token>) -> Vec<Token> {
    let mut output = vec![];
    for (i, token) in tokens.iter().enumerate() {
        let implied_start = match token {
            Token::Number(_) | Token::Identifier(_) | Token::CloseParen => true,
            _ => false,
        };
        let invalid = matches!(token, &Token::CloseParen) || matches!(token, &Token::Builtin(_));
        let implied_end = i + 1 < tokens.len()
            && match tokens[i + 1] {
                Token::Number(_) | Token::Identifier(_) | Token::Builtin(_) => true,
                Token::OpenParen if !invalid => true,
                _ => false,
            };

        output.push(token.clone());
        let closing = i + 1 < tokens.len() && matches!(&tokens[i + 1], &Token::CloseParen);
        if implied_start && implied_end && !closing {
            output.push(Token::Operator('*'));
        }
    }
    output
}

pub fn tokenize(expr: &str) -> Vec<Token> {
    let is_number = |v: char| v.is_numeric() || v == '.';
    let is_unknown = |v: char| {
        !['+', '-', '*', '/', '^', ')', '(', '='].contains(&v)
            && !is_number(v)
            && !v.is_whitespace()
    };

    let (mut num_start, mut id_start) = (0, 0);
    let mut prev: Option<char> = None;
    let mut tokens: Vec<Token> = Vec::new();
    let mut iterator = expr.char_indices().enumerate().peekable();

    while let Some((i, (_, c))) = iterator.next() {
        match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '+' | '-' | '*' | '/' | '^' | '=' => tokens.push(Token::Operator(c)),
            c if is_number(c) => {
                let next = iterator.peek();

                if prev.is_none() || !is_number(prev.unwrap()) {
                    num_start = i;
                }

                if next.is_none() || !is_number(next.unwrap().1.1) {
                    let slice = &expr[num_start..i + 1];
                    tokens.push(Token::Number(slice.parse::<f64>().unwrap()));
                }
            }
            c if !c.is_whitespace() => {
                if prev.is_none() || !is_unknown(prev.unwrap()) {
                    id_start = i;
                }

                let next = iterator.peek();
                let slice = slice_string(expr, id_start, i);
                let (builtin_func, builtin_const) = is_builtin(&slice);

                if let Some(func) = builtin_func {
                    tokens.push(Token::Builtin(func.to_string()));
                    id_start = i + 1;
                } else if let Some(num) = builtin_const {
                    tokens.push(Token::Number(num));
                    id_start = i + 1;
                } else if next.is_none() || !is_unknown(next.unwrap().1.1) {
                    // identifiers longer than 1 character are only allowed with subscripts. Otherwise,
                    // they are to be treated as several variables implicitly multipled together.
                    let slice = slice_string(expr, id_start, i);
                    let len = slice.chars().count();
                    if slice.contains("_") || len == 1 {
                        tokens.push(Token::Identifier(slice));
                    } else {
                        for (i, c) in slice.chars().enumerate() {
                            tokens.push(Token::Identifier(c.to_string()));
                            if i != len - 1 {
                                tokens.push(Token::Operator('*'))
                            }
                        }
                    }
                }
            }
            _ => {}
        };
        prev = Some(c);
    }

    insert_implied_tokens(&tokens)
}

#[derive(Debug, Default, PartialEq)]
pub enum OpType {
    #[default]
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Equal,
}

fn operator_info(c: &char, is_unary: bool) -> (OpType, u64) {
    match c {
        '=' => (OpType::Equal, 0),
        '+' => (OpType::Add, 5),
        '-' if !is_unary => (OpType::Sub, 5),
        '*' => (OpType::Mul, 10),
        '/' => (OpType::Div, 10),
        '^' => (OpType::Exp, 20),
        '-' if is_unary => (OpType::Sub, 30),
        _ => (OpType::Add, 0),
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum Expr {
    #[default]
    Placeholder,
    Operator {
        lhs: Box<Expr>,
        rhs: Option<Box<Expr>>,
        op: OpType,
    },
    Number {
        value: f64,
    },
    Identifier {
        value: String,
    },
    Function {
        name: String,
        arg: Box<Expr>,
        expr: Option<Box<Expr>>,
    },
}

pub fn parse_expr(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    precedence: u64,
) -> Result<Expr, String> {
    let mut node = Expr::default();

    while let Some(token) = tokens.next() {
        if let Token::CloseParen = token {
            break; // End of a group
        }

        match token {
            Token::Number(value) => node = Expr::Number { value },
            Token::Identifier(value) => node = Expr::Identifier { value },
            Token::Builtin(value) => node = Expr::Identifier { value }, // TODO: handle functions as unary ops
            Token::OpenParen => node = parse_expr(tokens, 0)?,
            Token::Operator(c) => {
                let can_be_unary = matches!(node, Expr::Placeholder) && c == '-';
                let (op, p) = operator_info(&c, can_be_unary);

                node = if can_be_unary {
                    Expr::Operator {
                        lhs: Box::new(parse_expr(tokens, p)?),
                        rhs: None,
                        op,
                    }
                } else {
                    Expr::Operator {
                        lhs: Box::new(node),
                        rhs: Some(Box::new(parse_expr(tokens, p)?)),
                        op,
                    }
                };
            }
            _ => {}
        }

        if let Some(t) = tokens.peek() {
            let p = match t {
                Token::Operator(c) => operator_info(c, false).1,
                _ => 0,
            };
            if p < precedence {
                break; // Stop accumulating on the left hand side
            }
        }
    }

    Ok(node)
}

fn main() {}

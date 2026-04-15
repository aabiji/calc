/*
- parse using precedence (for implicit operations as well)

- parse functions and their arguments

- handle sin notation (ex: sin 1/2x = sin(1/2x)

- write and pass more tests

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
                    tokens.push(Token::Identifier(func.to_string()));
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
                            if i != len {
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

    tokens
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
        '+' => (OpType::Add, 0),
        '-' if !is_unary => (OpType::Sub, 0),
        '*' => (OpType::Mul, 1),
        '/' => (OpType::Div, 1),
        '^' => (OpType::Exp, 2),
        '=' => (OpType::Equal, 3),
        '-' if is_unary => (OpType::Sub, 4),
        _ => (OpType::Add, 0),
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum Expr {
    #[default]
    Placeholder,
    Unary {
        arg: Box<Expr>,
        op: OpType,
    },
    Binary {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: OpType,
    },
    Number {
        value: f64,
    },
    Identifier {
        value: String,
    },
}

fn parse_operation(
    c: char,
    lhs: Option<Expr>,
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Expr, String> {
    let (op, p) = operator_info(&c, lhs.is_none());
    let rhs = parse_expr(tokens, p)?;
    if let Some(node) = lhs {
        Ok(Expr::Binary {
            lhs: Box::new(node),
            rhs: Box::new(rhs),
            op,
        })
    } else {
        Ok(Expr::Unary {
            arg: Box::new(rhs),
            op,
        })
    }
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
            Token::Number(value) => {
                node = Expr::Number { value };

                // Treat as an implied multiplication
                if matches!(
                    tokens.peek(),
                    Some(Token::OpenParen) | Some(Token::Identifier(_)) | Some(Token::Number(_)),
                ) {
                    node = parse_operation('*', Some(node), tokens)?;
                }
            }
            Token::Identifier(value) => node = Expr::Identifier { value },
            Token::OpenParen => {
                // Entering another expressoin resets precedence
                node = parse_expr(tokens, 0)?;
            }
            Token::Operator(c) => {
                let can_be_unary = matches!(node, Expr::Placeholder) && c == '-'; // TODO: handle functions as unary ops
                node = parse_operation(c, if can_be_unary { None } else { Some(node) }, tokens)?;
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

fn main() {
    //let expr = "123.0 + 133. * 1 / sin(x + 1) - ln(x^2 + 3)";
    //let expr = "1 * 2 + 3 + 4";
    //let expr = "-1 + 2 * 3 + 4 / 2";
    //let expr = "1 + 2f(x)";
    //let expr = "1 + 2f(x)";
    //let expr = "10 - 3sin(2x) + ln(x+2) / 5";
    //let expr = "1 = (2 + x) + 3";
    //let expr = "1 = (2 = x) + 3"; // TODO: catch errors like this!
    //let expr = "3xyz + 123 / 2(2)";
}

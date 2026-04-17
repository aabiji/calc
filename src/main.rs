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
        ("log", 0),
    ]);

    let builtin_consts: HashMap<&str, f64> =
        HashMap::from([("π", std::f64::consts::PI), ("e", std::f64::consts::E)]);

    (
        builtin_functions.get(value).map(|_| value),
        builtin_consts.get(value).cloned(),
    )
}

// Insert Token::Operator('*') between tokens that are implicitly
// multiplied together. Insert opening and closing parentheses around
// arguments that immediately follow the builtin functions. Arguments
// are only grouped by implicit multiplication, so for instance
// cos1/2x will be interpreted as (cos1) / 2x.
fn insert_implied_tokens(tokens: &Vec<Token>) -> Vec<Token> {
    let mut output = vec![];
    let mut have_implied_argument = false;

    for (i, token) in tokens.iter().enumerate() {
        let mut current = token.clone();
        if have_implied_argument && matches!(token, &Token::Operator(_) | &Token::OpenParen) {
            output.push(Token::CloseParen);
            current = Token::CloseParen;
            have_implied_argument = false;
        }

        let can_split = match &token {
            &Token::Identifier(id) => id.len() > 1 && !id.contains("_"),
            _ => false,
        };

        if let Token::Identifier(id) = &token
            && can_split
        {
            let chars = id.chars().enumerate();
            let len = chars.clone().count() - 1;
            for (i, c) in chars {
                output.push(Token::Identifier(c.to_string()));
                current = Token::Identifier(c.to_string());
                if i != len {
                    output.push(Token::Operator('*'));
                }
            }
        } else {
            if let Token::CloseParen = current
                && tokens[i] == Token::OpenParen
            {
                output.push(Token::Operator('*'));
            }

            current = token.clone();
            output.push(token.clone());
        }

        if i == tokens.len() - 1 {
            continue;
        }
        let next = &tokens[i + 1];

        let implied_mul1 = matches!(
            current,
            Token::Number(_) | Token::Identifier(_) | Token::CloseParen
        ) && matches!(
            next,
            &Token::Number(_) | &Token::Identifier(_) | &Token::Builtin(_)
        );

        let implied_mul2 = matches!(current, Token::Number(_) | Token::CloseParen)
            && matches!(next, &Token::OpenParen);

        let implied_argumnet =
            matches!(current, Token::Builtin(_)) && !matches!(next, &Token::OpenParen);

        if implied_argumnet {
            have_implied_argument = true;
            output.push(Token::OpenParen);
        }

        if implied_mul1 || implied_mul2 {
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
                    tokens.push(Token::Identifier(slice_string(expr, id_start, i)));
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
    Factorial,
    Equal,
}

// Returns (OpType, precedence, is_right_associative)
fn operator_info(c: &char, is_unary: bool) -> (OpType, u64, bool) {
    match c {
        '=' => (OpType::Equal, 1, true),
        '+' => (OpType::Add, 2, false),
        '-' if !is_unary => (OpType::Sub, 2, false),
        '-' if is_unary => (OpType::Sub, 4, true),
        '*' => (OpType::Mul, 3, false),
        '/' => (OpType::Div, 3, false),
        '^' => (OpType::Exp, 5, true),
        '!' => (OpType::Factorial, 4, true),
        _ => (OpType::Add, 0, false),
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
    },
}

// Have no idea why there's a bug, so I have the option of rewriting the parser.
// -> parse_single will parse the base tokens like num, identifier, etc.
// -> parse_expr will parse operators only if the next operator's precedence is >= the current one
pub fn parse_expr(tokens: &mut Peekable<impl Iterator<Item = Token>>, precedence: u64) -> Expr {
    let mut node = Expr::default();

    while let Some(token) = tokens.next() {
        match token {
            Token::Number(value) => node = Expr::Number { value },
            Token::OpenParen => node = parse_expr(tokens, 0),
            Token::Identifier(value) => {
                node = if matches!(tokens.peek(), Some(Token::OpenParen)) {
                    tokens.next(); // Skip the Token::OpenParen
                    Expr::Function {
                        name: value,
                        arg: Box::new(parse_expr(tokens, 0)),
                    }
                } else {
                    Expr::Identifier { value }
                };
            }
            Token::Builtin(value) => {
                tokens.next(); // Skip the Token::OpenParen
                node = Expr::Function {
                    name: value,
                    arg: Box::new(parse_expr(tokens, 0)),
                };
            }
            Token::Operator(c) => {
                // Nodes accumulate on the right hand side of an operator expression
                let can_be_unary = matches!(node, Expr::Placeholder) && c == '-';
                let (op, p, right_associative) = operator_info(&c, can_be_unary);
                let p = if right_associative { p - 1 } else { p };

                node = if can_be_unary {
                    Expr::Operator {
                        lhs: Box::new(parse_expr(tokens, p)),
                        rhs: None,
                        op,
                    }
                } else {
                    Expr::Operator {
                        lhs: Box::new(node),
                        rhs: Some(Box::new(parse_expr(tokens, p))),
                        op,
                    }
                };
            }
            _ => {}
        }

        if let Some(t) = tokens.peek() {
            let p = match t {
                Token::Operator(c) => operator_info(c, false).1,
                Token::CloseParen => {
                    tokens.next(); // Skip the closing ')'
                    0
                },
                _ => 0,
            };
            if p <= precedence {
                break; // Stop accumulating the node
            }
        }
    }

    node
}

fn main() {}

use std::collections::HashMap;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
enum Token {
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

fn tokenize(expr: &str) -> Vec<Token> {
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
                    tokens.push(Token::Identifier(slice_string(expr, id_start, i)));
                }
            }
            _ => {}
        };
        prev = Some(c);
    }

    tokens
}

#[derive(Debug, Default)]
enum NodeType {
    #[default]
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Equal,
}

#[derive(Debug, Default)]
struct Node {
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
    op: NodeType,
    id: String,
    value: f64,
}

fn parse_binary_expr(
    c: char,
    lhs: Node,
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Node, String> {
    let op = match c {
        '+' => NodeType::Add,
        '-' => NodeType::Sub,
        '*' => NodeType::Mul,
        '/' => NodeType::Div,
        '^' => NodeType::Exp,
        '=' => NodeType::Equal,
        _ => return Err("Uknown operator".to_string()),
    };

    let rhs = parse_expr(tokens)?;
    Ok(Node {
        lhs: Some(Box::new(lhs)),
        rhs: Some(Box::new(rhs)),
        op,
        ..Default::default()
    })
}

fn parse_expr(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Node, String> {
    let mut node = Node::default();

    while let Some(token) = tokens.next() {
        if let Token::CloseParen = token {
            break;
        }

        match token {
            Token::Number(value) => {
                node = Node {
                    value,
                    ..Default::default()
                }
            }
            Token::Identifier(id) => {
                node = Node {
                    id,
                    ..Default::default()
                }
            }
            Token::OpenParen => {
                node = parse_expr(tokens)?;
            }
            Token::Operator(c) => {
                node = parse_binary_expr(c, node, tokens)?;
            }
            _ => {}
        }
    }

    Ok(node)
}

fn main() {
    let expr = "123.0 + 133. * 1 / sin(x + 1) - ln(x^2 + 3)";
    let expr = "1 * 2 + 3 + 4";
    let expr = "-1 + 2 * 3 + 4 / 2";
    let expr = "4^5 + 2^(x^2 + 1)";
    let expr = "1 + 2f(x)";
    let expr = "1 + 2f(x)";
    let expr = "10 - 3sin(2x) + ln(x+2) / 5";
    //let expr = "1 = (2 = x) + 3"; // TODO: catch errors like this!
    //let expr = "1 = (2 - x) + 3";

    let mut iter = tokenize(expr).into_iter().peekable();
    match parse_expr(&mut iter) {
        Result::Ok(root) => {
            dbg!(&root);
        }
        Result::Err(err) => println!("ERROR: {err}"),
    };
}

mod tests {
    use crate::Token;

    #[test]
    fn test_constants() {
        let pi = std::f64::consts::PI;
        let e = std::f64::consts::E;
        assert_eq!(
            crate::tokenize("π + e"),
            vec![Token::Number(pi), Token::Operator('+'), Token::Number(e)]
        );
    }

    #[test]
    fn test_functions() {
        let expected = vec![
            Token::Number(10.0),
            Token::Operator('-'),
            Token::Number(3.0),
            Token::Identifier(String::from("sin")),
            Token::Identifier(String::from("x")),
            Token::OpenParen,
            Token::Number(2.0),
            Token::Identifier(String::from("x")),
            Token::CloseParen,
        ];
        assert_eq!(crate::tokenize("10 - 3sinx(2x)"), expected);
    }
}

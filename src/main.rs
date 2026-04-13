use std::iter::Peekable;

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Operator(char),
    Identifier(String),
    OpenParen,
    CloseParen,
    Equal,
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
    let mut iterator = expr.char_indices().peekable();

    while let Some((i, c)) = iterator.next() {
        match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '=' => tokens.push(Token::Equal),
            '+' | '-' | '*' | '/' | '^' => tokens.push(Token::Operator(c)),
            c if is_number(c) => {
                let next = iterator.peek();

                if prev.is_none() || !is_number(prev.unwrap()) {
                    num_start = i;
                }

                if next.is_none() || !is_number(next.unwrap().1) {
                    let slice = &expr[num_start..i + 1];
                    tokens.push(Token::Number(slice.parse::<f64>().unwrap()));
                }
            }
            c if !c.is_whitespace() => {
                let next = iterator.peek();

                if prev.is_none() || !is_unknown(prev.unwrap()) {
                    id_start = i;
                }

                if next.is_none() || !is_unknown(next.unwrap().1) {
                    tokens.push(Token::Identifier(expr[id_start..i + 1].to_string()));
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
    Group,
    Add,
    Sub,
    Mul,
    Div,
    Exp,
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
    lhs: Node,
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Node, String> {
    match tokens.next() {
        Some(Token::Operator(c)) => {
            let op = match c {
                '+' => NodeType::Add,
                '-' => NodeType::Sub,
                '*' => NodeType::Mul,
                '/' => NodeType::Div,
                '^' => NodeType::Exp,
                _ => return Err("Uknown operator".to_string()),
            };

            let rhs = parse_expr(tokens)?;
            return Ok(Node {
                lhs: Some(Box::new(lhs)),
                rhs: Some(Box::new(rhs)),
                op,
                ..Default::default()
            });
        }
        _ => return Err("Expected an operator".to_string()),
    }
}

fn parse_expr(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Node, String> {
    let mut node = Node {
        op: NodeType::Group,
        ..Node::default()
    };

    while let Some(token) = tokens.next() {
        if let Token::CloseParen = token {
            return Ok(node);
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
            _ => {}
        }

        if let Some(Token::Operator(_)) = tokens.peek() {
            node = parse_binary_expr(node, tokens)?;
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
    let expr = "10 - 3sin(2x)";
    let expr = "1 + x";

    let mut iter = tokenize(expr).into_iter().peekable();
    match parse_expr(&mut iter) {
        Result::Ok(root) => {
            dbg!(&root);
        }
        Result::Err(err) => println!("ERROR: {err}"),
    };
}

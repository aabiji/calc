#[cfg(test)]
use crate::{Node, OpType, Token, parse_expr, tokenize};

#[test]
fn test_constants() {
    let pi = std::f64::consts::PI;
    let e = std::f64::consts::E;
    assert_eq!(
        tokenize("π + e"),
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
    assert_eq!(tokenize("10 - 3sinx(2x)"), expected);
}

#[test]
fn test_ops() {
    let expr = "-1 = (2 + x) + 3";
    let mut tokens = tokenize(expr).into_iter().peekable();
    assert_eq!(
        parse_expr(&mut tokens, 0),
        Ok(Node::Binary {
            lhs: Box::new(Node::Unary {
                arg: Box::new(Node::Number { value: 1.0 }),
                op: OpType::Sub
            }),
            rhs: Box::new(Node::Binary {
                lhs: Box::new(Node::Binary {
                    lhs: Box::new(Node::Number { value: 2.0 }),
                    rhs: Box::new(Node::Identifier {
                        value: String::from("x")
                    }),
                    op: OpType::Add,
                }),
                rhs: Box::new(Node::Number { value: 3.0 }),
                op: OpType::Add,
            }),
            op: OpType::Equal,
        })
    );
}

#[test]
fn test_parse_precedence() {
    let expr = "5 * 3 + 2 / 1";
    let mut tokens = tokenize(expr).into_iter().peekable();
    debug_assert_eq!(
        parse_expr(&mut tokens, 0),
        Ok(Node::Binary {
            lhs: Box::new(Node::Binary {
                lhs: Box::new(Node::Number { value: 5.0 }),
                rhs: Box::new(Node::Number { value: 3.0 }),
                op: OpType::Mul
            }),

            rhs: Box::new(Node::Binary {
                lhs: Box::new(Node::Number { value: 2.0 }),
                rhs: Box::new(Node::Number { value: 1.0 }),
                op: OpType::Div,
            }),
            op: OpType::Add,
        })
    );
}

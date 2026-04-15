#[cfg(test)]
use crate::{Expr, OpType, Token, parse_expr, tokenize};

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
fn test_parse_precedence() {
    let expr = "4 ^ 5 * 3 + 2 ^ (-x / 2 + 1)";
    let mut tokens = tokenize(expr).into_iter().peekable();

    assert_eq!(
        parse_expr(&mut tokens, 0),
        Ok(Expr::Binary {
            lhs: Box::new(Expr::Binary {
                lhs: Box::new(Expr::Binary {
                    lhs: Box::new(Expr::Number { value: 4.0 }),
                    rhs: Box::new(Expr::Number { value: 5.0 }),
                    op: OpType::Exp
                }),
                rhs: Box::new(Expr::Number { value: 3.0 }),
                op: OpType::Mul
            }),
            rhs: Box::new(Expr::Binary {
                lhs: Box::new(Expr::Number { value: 2.0 }),
                rhs: Box::new(Expr::Binary {
                    lhs: Box::new(Expr::Binary {
                        lhs: Box::new(Expr::Unary {
                            arg: Box::new(Expr::Identifier {
                                value: String::from("x")
                            }),
                            op: OpType::Sub
                        }),
                        rhs: Box::new(Expr::Number { value: 2.0 }),
                        op: OpType::Div
                    }),
                    rhs: Box::new(Expr::Number { value: 1.0 }),
                    op: OpType::Add
                }),
                op: OpType::Exp
            }),
            op: OpType::Add
        })
    );
}

#[test]
fn test_ops() {
    let expr = "-1 = (2 + x) + 3";
    let mut tokens = tokenize(expr).into_iter().peekable();
    assert_eq!(
        parse_expr(&mut tokens, 0),
        Ok(Expr::Binary {
            lhs: Box::new(Expr::Unary {
                arg: Box::new(Expr::Number { value: 1.0 }),
                op: OpType::Sub
            }),
            rhs: Box::new(Expr::Binary {
                lhs: Box::new(Expr::Binary {
                    lhs: Box::new(Expr::Number { value: 2.0 }),
                    rhs: Box::new(Expr::Identifier {
                        value: String::from("x")
                    }),
                    op: OpType::Add,
                }),
                rhs: Box::new(Expr::Number { value: 3.0 }),
                op: OpType::Add,
            }),
            op: OpType::Equal,
        })
    );
}

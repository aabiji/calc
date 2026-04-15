#[cfg(test)]
use crate::{Expr, OpType, Token, parse_expr, tokenize};

#[test]
fn test_constant_tokens() {
    let pi = std::f64::consts::PI;
    let e = std::f64::consts::E;
    assert_eq!(
        tokenize("π + e"),
        vec![Token::Number(pi), Token::Operator('+'), Token::Number(e)]
    );
}

#[test]
fn test_function_tokens() {
    let expected = vec![
        Token::Number(10.0),
        Token::Operator('-'),
        Token::Number(3.0),
        Token::Operator('*'),
        Token::Builtin(String::from("sin")),
        Token::Identifier(String::from("x")),
        Token::Operator('*'),
        Token::OpenParen,
        Token::Number(2.0),
        Token::Operator('*'),
        Token::Identifier(String::from("x")),
        Token::CloseParen,
    ];
    assert_eq!(tokenize("10 - 3sinx(2x)"), expected);
}

#[test]
fn parse_precedence() {
    let mut tokens = tokenize("4 ^ 5 * 3 + 2 ^ (-x / 2 + 1)")
        .into_iter()
        .peekable();

    assert_eq!(
        parse_expr(&mut tokens, 0),
        Ok(Expr::Operator {
            lhs: Box::new(Expr::Operator {
                lhs: Box::new(Expr::Operator {
                    lhs: Box::new(Expr::Number { value: 4.0 }),
                    rhs: Some(Box::new(Expr::Number { value: 5.0 })),
                    op: OpType::Exp
                }),
                rhs: Some(Box::new(Expr::Number { value: 3.0 })),
                op: OpType::Mul
            }),
            rhs: Some(Box::new(Expr::Operator {
                lhs: Box::new(Expr::Number { value: 2.0 }),
                rhs: Some(Box::new(Expr::Operator {
                    lhs: Box::new(Expr::Operator {
                        lhs: Box::new(Expr::Operator {
                            lhs: Box::new(Expr::Identifier {
                                value: String::from("x")
                            }),
                            rhs: None,
                            op: OpType::Sub
                        }),
                        rhs: Some(Box::new(Expr::Number { value: 2.0 })),
                        op: OpType::Div
                    }),
                    rhs: Some(Box::new(Expr::Number { value: 1.0 })),
                    op: OpType::Add
                })),
                op: OpType::Exp
            })),
            op: OpType::Add
        })
    );
}

#[test]
fn parse_operators() {
    let mut tokens = tokenize("-1(2) + 6 = (2 + 2xyz) + 3")
        .into_iter()
        .peekable();
    assert_eq!(
        parse_expr(&mut tokens, 0),
        Ok(Expr::Operator {
            lhs: Box::new(Expr::Operator {
                lhs: Box::new(Expr::Operator {
                    lhs: Box::new(Expr::Operator {
                        lhs: Box::new(Expr::Number { value: 1.0 }),
                        rhs: None,
                        op: OpType::Sub
                    }),
                    rhs: Some(Box::new(Expr::Number { value: 2.0 })),
                    op: OpType::Mul
                }),
                rhs: Some(Box::new(Expr::Number { value: 6.0 })),
                op: OpType::Add
            }),
            rhs: Some(Box::new(Expr::Operator {
                lhs: Box::new(Expr::Operator {
                    lhs: Box::new(Expr::Number { value: 2.0 }),
                    rhs: Some(Box::new(Expr::Operator {
                        lhs: Box::new(Expr::Number { value: 2.0 }),
                        rhs: Some(Box::new(Expr::Operator {
                            lhs: Box::new(Expr::Identifier {
                                value: String::from("x")
                            }),
                            rhs: Some(Box::new(Expr::Operator {
                                lhs: Box::new(Expr::Identifier {
                                    value: String::from("y")
                                }),
                                rhs: Some(Box::new(Expr::Identifier {
                                    value: String::from("z")
                                })),
                                op: OpType::Mul
                            })),
                            op: OpType::Mul
                        })),
                        op: OpType::Mul
                    })),
                    op: OpType::Add,
                }),
                rhs: Some(Box::new(Expr::Number { value: 3.0 })),
                op: OpType::Add,
            })),
            op: OpType::Equal,
        })
    );
}

#[test]
fn parse_functions() {
    let mut tokens = tokenize("f(x) = y").into_iter().peekable();
    assert_eq!(
        parse_expr(&mut tokens, 0),
        Ok(Expr::Function {
            name: String::from("f"),
            arg: Box::new(Expr::Identifier {
                value: String::from("x")
            }),
            expr: Some(Box::new(Expr::Identifier {
                value: String::from("y")
            }))
        }),
    );

    let e = std::f64::consts::E;
    let mut tokens = tokenize("f(x) = 5ln(e + 3) + sin3x + cos1/2x + cos(1/2x) + 2g(x)")
        .into_iter()
        .peekable();
    assert_eq!(
        parse_expr(&mut tokens, 0),
        Ok(Expr::Function {
            name: String::from("f"),
            arg: Box::new(Expr::Identifier {
                value: String::from("x")
            }),
            expr: Some(Box::new(Expr::Operator {
                lhs: Box::new(Expr::Operator {
                    lhs: Box::new(Expr::Operator {
                        lhs: Box::new(Expr::Operator {
                            lhs: Box::new(Expr::Operator {
                                lhs: Box::new(Expr::Number { value: 5.0 }),
                                rhs: Some(Box::new(Expr::Function {
                                    name: String::from("ln"),
                                    arg: Box::new(Expr::Operator {
                                        lhs: Box::new(Expr::Number { value: e }),
                                        rhs: Some(Box::new(Expr::Number { value: 3.0 })),
                                        op: OpType::Add
                                    }),
                                    expr: None
                                })),
                                op: OpType::Mul
                            }),
                            rhs: Some(Box::new(Expr::Function {
                                name: String::from("sin"),
                                arg: Box::new(Expr::Operator {
                                    lhs: Box::new(Expr::Number { value: 3.0 }),
                                    rhs: Some(Box::new(Expr::Identifier {
                                        value: String::from("x")
                                    })),
                                    op: OpType::Mul
                                }),
                                expr: None
                            })),
                            op: OpType::Add
                        }),
                        rhs: Some(Box::new(Expr::Function {
                            name: String::from("cos"),
                            arg: Box::new(Expr::Operator {
                                lhs: Box::new(Expr::Operator {
                                    lhs: Box::new(Expr::Number { value: 1.0 }),
                                    rhs: Some(Box::new(Expr::Number { value: 2.0 })),
                                    op: OpType::Div
                                }),
                                rhs: Some(Box::new(Expr::Identifier {
                                    value: String::from("x")
                                })),
                                op: OpType::Mul
                            }),
                            expr: None
                        })),
                        op: OpType::Add
                    }),
                    rhs: Some(Box::new(Expr::Function {
                        name: String::from("cos"),
                        arg: Box::new(Expr::Operator {
                            lhs: Box::new(Expr::Operator {
                                lhs: Box::new(Expr::Number { value: 1.0 }),
                                rhs: Some(Box::new(Expr::Number { value: 2.0 })),
                                op: OpType::Div
                            }),
                            rhs: Some(Box::new(Expr::Identifier {
                                value: String::from("x")
                            })),
                            op: OpType::Mul
                        }),
                        expr: None
                    })),
                    op: OpType::Add
                }),
                rhs: Some(Box::new(Expr::Operator {
                    lhs: Box::new(Expr::Number { value: 2.0 }),
                    rhs: Some(Box::new(Expr::Function {
                        name: String::from("g"),
                        arg: Box::new(Expr::Identifier {
                            value: String::from("x")
                        }),
                        expr: None
                    })),
                    op: OpType::Mul
                })),
                op: OpType::Add
            }))
        })
    );
}

use core::panic;

use super::token::{self, Token, TokenIter};
use super::parse_error::ParseError;
use crate::expr::Expr;
use super::macros::expect_token;

type DependentOnVar = bool;

pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut tokens = tokens.into_iter().peekable();
    parse_expr(&mut tokens).0  
}

fn parse_expr(tokens: &mut TokenIter) -> (Expr, DependentOnVar) {
    parse_sum(tokens)
}

fn parse_sum(tokens: &mut TokenIter) -> (Expr, DependentOnVar) {
    let (mut lhs, lhs_dep) = parse_product(tokens);
    let mut expr_dep = lhs_dep;

    while let Some(token) = tokens.peek() {
        expr_dep = expr_dep && lhs_dep;

        match token {
            Token::Plus | Token::Minus => {
                let token = tokens.next().unwrap();
                
                let (rhs, rhs_dep) = parse_product(tokens);
                lhs = check_dependence(&token, lhs, rhs, lhs_dep && rhs_dep);
            },

            _ => break,
        }
    }

    (lhs, expr_dep)
}

fn parse_product(tokens: &mut TokenIter) -> (Expr, DependentOnVar) {
    let (mut lhs, lhs_dep) = parse_atom(tokens);
    let mut expr_dep = lhs_dep;

    while let Some(token) = tokens.peek() {
        expr_dep = expr_dep && lhs_dep;

        match token {
            Token::Slash | Token::Star => {
                let token = tokens.next().unwrap();

                let (rhs, rhs_dep) = parse_atom(tokens);
                lhs = check_dependence(&token, lhs, rhs, lhs_dep && rhs_dep);
            },

            _ => break,
        }
    }

    (lhs, expr_dep)
}

fn parse_atom(tokens: &mut TokenIter) -> (Expr, DependentOnVar) {
    match tokens.peek().unwrap() {
        Token::Number(_) => {
            expect_token!(Token::Number(n) in ITER tokens);
            (Expr::Num(n), false)
        },

        Token::Ident(_) => {
            expect_token!(Token::Ident(s) in ITER tokens);
            (Expr::Var(s), true)
        },
        
        Token::LParen => {
            expect_token!(Token::LParen in ITER tokens);
            let inner = parse_expr(tokens);
            expect_token!(Token::RParen in ITER tokens);
            inner
        },

        _ => panic!("Unexpected token"),
    }
}

fn check_dependence(
    token: &Token,
    lhs: Expr,
    rhs: Expr,
    dep: bool,
) -> Expr {
    if dep {
        let result = op_token_apply_unchecked(token, lhs, rhs);
        Expr::Num(result)
    } else {
        op_token_to_expr_unchecked(token, lhs, rhs)
    }
}

fn op_token_apply_unchecked(token: &Token, lhs: Expr, rhs: Expr) -> i32 {
    let lhs = lhs.eval_const();
    let rhs = rhs.eval_const();
    match token {
        Token::Plus => lhs + rhs,
        Token::Minus => lhs - rhs,
        Token::Star => lhs * rhs,
        Token::Slash =>  lhs / rhs,
        _ => panic!("Unexpected token"),
    }
}

fn op_token_to_expr_unchecked(token: &Token, lhs: Expr, rhs: Expr) -> Expr {
    match token {
        Token::Plus => Expr::Add(Box::new(lhs), Box::new(rhs)),
        Token::Minus => Expr::Sub(Box::new(lhs), Box::new(rhs)),
        Token::Star => Expr::Mul(Box::new(lhs), Box::new(rhs)),
        Token::Slash => Expr::Div(Box::new(lhs), Box::new(rhs)),
        _ => panic!("Unexpected token"),
    }
}
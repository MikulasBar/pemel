use core::panic;
use std::any;

use super::token::{self, Token, TokenIter};
use super::parse_error::ParseError;
use crate::expr::Expr;
use super::macros::expect_token;

type IsConst = bool;

pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut tokens = tokens.into_iter().peekable();
    parse_expr(&mut tokens).0  
}

fn parse_expr(tokens: &mut TokenIter) -> (Expr, IsConst) {
    parse_sum(tokens)
}

#[allow(irrefutable_let_patterns)]
fn parse_sum(tokens: &mut TokenIter) -> (Expr, IsConst) {
    let (mut lhs, mut is_lhs_const) = parse_product(tokens);

    while let Some(Token::Plus | Token::Minus) = tokens.peek() {
        expect_token!(token in ITER tokens);
        
        let (rhs, is_rhs_const) = parse_product(tokens);
        let is_both_const = is_lhs_const && is_rhs_const;
        lhs = merge_by_dependence(&token, lhs, rhs, is_both_const);
        is_lhs_const = is_both_const;
    }

    (lhs, is_lhs_const)
}

#[allow(irrefutable_let_patterns)]
fn parse_product(tokens: &mut TokenIter) -> (Expr, IsConst) {
    let (mut lhs, mut is_lhs_const) = parse_atom(tokens);

    while let Some(Token::Star | Token::Slash) = tokens.peek() {
        expect_token!(token in ITER tokens);
        
        let (rhs, is_rhs_const) = parse_atom(tokens);
        let is_both_const = is_lhs_const && is_rhs_const;
        lhs = merge_by_dependence(&token, lhs, rhs, is_both_const);
        is_lhs_const = is_both_const;
    }

    (lhs, is_lhs_const)
}

fn parse_atom(tokens: &mut TokenIter) -> (Expr, IsConst) {
    let (lhs, is_lhs_const) = match tokens.peek().unwrap() {
        Token::Number(_) => {
            expect_token!(Token::Number(n) in ITER tokens);
            (Expr::Num(n), true)
        },

        Token::Ident(_) => {
            expect_token!(Token::Ident(s) in ITER tokens);
            (Expr::Var(s), false)
        },
        
        Token::LParen => parse_parens(tokens),

        Token::Sin => {
            expect_token!(Token::Sin in ITER tokens);
            let (inner, is_inner_const) = parse_parens(tokens);

            if is_inner_const {
                (Expr::Num(inner.eval_const().sin()), true)
            } else {
                (Expr::new_sin(inner), false)
            }
        }

        _ => panic!("Unexpected token"),
    };

    parse_implicit_multiplication(lhs, is_lhs_const, tokens)
}

fn merge_by_dependence(
    token: &Token,
    lhs: Expr,
    rhs: Expr,
    is_const: bool,
) -> Expr {
    if is_const {
        let result = op_token_apply_unchecked(token, lhs, rhs);
        Expr::Num(result)
    } else {
        op_token_to_expr_unchecked(token, lhs, rhs)
    }
}

fn op_token_apply_unchecked(token: &Token, lhs: Expr, rhs: Expr) -> f32 {
    let lhs = lhs.eval_const();
    let rhs = rhs.eval_const();
    match token {
        Token::Plus  => lhs + rhs,
        Token::Minus => lhs - rhs,
        Token::Star  => lhs * rhs,
        Token::Slash => lhs / rhs,
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

fn parse_implicit_multiplication(mut lhs: Expr, mut is_lhs_const: IsConst, tokens: &mut TokenIter) -> (Expr, IsConst) {
    while let Some(token) = tokens.peek() {
        match token {
            Token::Ident(_) => {
                expect_token!(Token::Ident(s) in ITER tokens);
    
                lhs = Expr::new_mul(lhs, Expr::Var(s));
                is_lhs_const = false;
            },
    
            Token::LParen => {
                let (inner, is_inner_const) = parse_parens(tokens);

                lhs = Expr::new_mul(lhs, inner);
                is_lhs_const = is_lhs_const && is_inner_const;
            },
    
            _ => break,
        }
    }

    (lhs, is_lhs_const)
}

fn parse_parens(tokens: &mut TokenIter) -> (Expr, IsConst) {
    expect_token!(Token::LParen in ITER tokens);
    let (inner, is_inner_const) = parse_expr(tokens);
    expect_token!(Token::RParen in ITER tokens);
    (inner, is_inner_const)
}
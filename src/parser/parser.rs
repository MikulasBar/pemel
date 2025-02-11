use core::f32;

use super::macros::expect_token;
use super::parse_error::ParseError;
use super::token::{Token, TokenIter};
use crate::expr::Expr;
use crate::parser::macros::expect_token_ret;

type IsConst = bool;
type ParseResult = Result<(Expr, IsConst), ParseError>;

pub fn parse(tokens: Vec<Token>, implicit_evaluation: bool) -> Result<Expr, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    let result = parse_expr(&mut tokens, implicit_evaluation).map(|x| x.0);

    if tokens.peek().is_some() {
        return Err(ParseError::UnexpectedToken(tokens.next().unwrap()));
    }

    result
}

fn parse_expr(tokens: &mut TokenIter, implicit_evaluation: bool) -> ParseResult {
    let result = parse_sum(tokens, implicit_evaluation);

    result
}

fn parse_sum(tokens: &mut TokenIter, implicit_evaluation: bool) -> ParseResult {
    parse_binop(
        |t| matches!(t, Some(Token::Plus | Token::Minus)),
        |iter| parse_product(iter, implicit_evaluation),
        tokens,
        implicit_evaluation
    )
}

fn parse_product(tokens: &mut TokenIter, implicit_evaluation: bool) -> ParseResult {
    parse_binop(
        |t| matches!(t, Some(Token::Star | Token::Slash)),
        |iter| parse_power(iter, implicit_evaluation),
        tokens,
        implicit_evaluation
    )
}

fn parse_power(tokens: &mut TokenIter, implicit_evaluation: bool) -> ParseResult {
    parse_binop(
        |t| matches!(t, Some(Token::Caret)),
        |iter| parse_atom(iter, implicit_evaluation),
        tokens,
        implicit_evaluation
    )
}

fn parse_binop(
    match_op: fn(Option<&Token>) -> bool,
    parse_prev: impl Fn(&mut TokenIter) -> ParseResult,
    tokens: &mut TokenIter,
    implicit_evaluation: bool,
) -> ParseResult {
    let (mut lhs, mut is_lhs_const) = parse_prev(tokens)?;

    while match_op(tokens.peek()) {
        expect_token!(token in ITER tokens);

        let (rhs, is_rhs_const) = parse_prev(tokens)?;
        let are_both_const = is_lhs_const && is_rhs_const;
        lhs = to_binop(&token, lhs, rhs, are_both_const && implicit_evaluation)?;
        is_lhs_const = are_both_const;
    }

    Ok((lhs, is_lhs_const))
}

fn parse_atom(tokens: &mut TokenIter, implicit_evaluation: bool) -> ParseResult {
    let sign = parse_sign(tokens);
    let atom = match tokens.peek().unwrap() {
        Token::LParen => parse_parens(tokens, implicit_evaluation),
        Token::Ident(_) => parse_ident(tokens, implicit_evaluation),

        Token::Number(_) => {
            expect_token!(Token::Number(n) in ITER tokens);
            Ok((Expr::Num(n), true))
        }

        _ => Err(ParseError::UnexpectedToken(tokens.next().unwrap())),
    };

    if sign == -1.0 {
        let (expr, is_const) = atom?;
        Ok((Expr::new_mul(-1.0, expr), is_const))
    } else {
        atom
    }
}

fn parse_sign(tokens: &mut TokenIter) -> f32 {
    let mut sign = 1.0;

    while let Some(&Token::Plus | &Token::Minus) = tokens.peek() {
        let token = tokens.next().unwrap();
        if let Token::Plus = token {
            sign *= -1.0;
        }
    }

    sign
}

fn parse_ident(tokens: &mut TokenIter, implicit_evaluation: bool) -> ParseResult {
    expect_token!(Token::Ident(ident) in ITER tokens);

    if let Some(Token::LParen) = tokens.peek() {
        expect_token!(Token::LParen in ITER tokens);
        let (args, is_const) = parse_args(tokens, implicit_evaluation)?;
        expect_token_ret!(Token::RParen in ITER tokens);

        let func = to_func(ident, args, is_const && implicit_evaluation)?;
        return Ok((func, is_const));
    }

    Ok((ident.into(), false))
}

fn parse_parens(tokens: &mut TokenIter, implicit_evaluation: bool) -> ParseResult {
    expect_token!(Token::LParen in ITER tokens);
    let result = parse_expr(tokens, implicit_evaluation);
    println!("{:?}", tokens.peek());
    expect_token_ret!(Token::RParen in ITER tokens);
    result
}

fn parse_args(tokens: &mut TokenIter, implicit_evaluation: bool) -> Result<(Vec<Expr>, IsConst), ParseError> {
    let mut args = vec![];
    let mut is_const = true;

    loop {
        let (arg, is_arg_const) = parse_expr(tokens, implicit_evaluation)?;
        args.push(arg);
        is_const = is_const && is_arg_const;

        if let Some(Token::Comma) = tokens.peek() {
            tokens.next();
        } else {
            break;
        }
    }

    Ok((args, is_const))
}

fn to_func(ident: String, args: Vec<Expr>, is_const: IsConst) -> Result<Expr, ParseError> {
    let func = wrap_with_func(ident, args)?;

    if is_const {
        let val = func
            .eval_const()
            .map_err(|err| ParseError::EvalError(err))?;

        Ok(val.into())
    } else {
        Ok(func)
    }
}

// TODO: Refactor this function
fn wrap_with_func(ident: String, mut args: Vec<Expr>) -> Result<Expr, ParseError> {
    use std::mem;

    let len = args.len();

    if len > 2 || args.is_empty() {
        return Err(ParseError::WrongNumberOfArgs(len));
    }

    let arg0 = mem::take(&mut args[0]);

    Ok(match (ident.as_str(), len) {
        ("sin", 1) => Expr::new_sin(arg0),
        ("cos", 1) => Expr::new_cos(arg0),
        ("tan", 1) => Expr::new_tan(arg0),
        ("cot", 1) => Expr::new_cot(arg0),
        ("abs", 1) => Expr::new_abs(arg0),
        ("ln", 1) => Expr::new_log(f32::consts::E, arg0),
        ("log", 1) => Expr::new_log(Expr::Num(10.0), arg0),

        ("log", 2) => {
            let arg1 = mem::take(&mut args[1]);
            Expr::new_log(arg0, arg1)
        }

        ("cos" | "sin" | "ln" | "log" | "tan" | "cot", _) => return Err(ParseError::WrongNumberOfArgs(len)),
        _ => return Err(ParseError::FunctionNotRecognized(ident)),
    })
}

fn to_binop(token: &Token, lhs: Expr, rhs: Expr, is_const: bool) -> Result<Expr, ParseError> {
    let expr = wrap_with_binop(token, lhs, rhs)?;

    if is_const {
        let val = expr
            .eval_const()
            .map_err(|err| ParseError::EvalError(err))?;

        Ok(val.into())
    } else {
        Ok(expr)
    }
}

fn wrap_with_binop(token: &Token, lhs: Expr, rhs: Expr) -> Result<Expr, ParseError> {
    Ok(match token {
        Token::Plus => Expr::new_add(lhs, rhs),
        Token::Minus => Expr::new_sub(lhs, rhs),
        Token::Star => Expr::new_mul(lhs, rhs),
        Token::Slash => Expr::new_div(lhs, rhs),
        Token::Caret => Expr::new_pow(lhs, rhs),
        _ => return Err(ParseError::UnexpectedToken(token.clone())),
    })
}

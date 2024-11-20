use super::token::{self, Token, TokenIter};
use super::parse_error::ParseError;
use super::macros::expect_token;
use crate::expr::Expr;
use crate::parser::macros::expect_token_ret;

type IsConst = bool;
type ParseResult = Result<(Expr, IsConst), ParseError>;

pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    let result = parse_expr(&mut tokens).map(|x| x.0);

    if tokens.peek().is_some() {
        return Err(ParseError::UnexpectedToken(tokens.next().unwrap()));
    }

    result
}

fn parse_expr(tokens: &mut TokenIter) -> ParseResult {
    let result = parse_sum(tokens);

    result
}


fn parse_sum(tokens: &mut TokenIter) -> ParseResult {
    parse_binop(
        |t| matches!(t, Some(Token::Plus | Token::Minus)),
        |iter| parse_product(iter),
        tokens
    )
}

fn parse_product(tokens: &mut TokenIter) -> ParseResult {
    parse_binop(
        |t| matches!(t, Some(Token::Star | Token::Slash)),
        |iter| parse_power(iter),
        tokens
    )
}

fn parse_power(tokens: &mut TokenIter) -> ParseResult {
    parse_binop(
        |t| matches!(t, Some(Token::Caret)),
        |iter| parse_atom(iter),
        tokens
    )
}

fn parse_binop(
    match_op: fn(Option<&Token>) -> bool,
    parse_prev: fn(&mut TokenIter) -> ParseResult,
    tokens: &mut TokenIter
) -> ParseResult {
    let (mut lhs, mut is_lhs_const) = parse_prev(tokens)?;

    while match_op(tokens.peek()) {
        expect_token!(token in ITER tokens);

        let (rhs, is_rhs_const) = parse_prev(tokens)?;
        let are_both_const = is_lhs_const && is_rhs_const;
        lhs = to_binop(&token, lhs, rhs, are_both_const);
        is_lhs_const = are_both_const;
    }

    Ok((lhs, is_lhs_const))
}

fn parse_atom(tokens: &mut TokenIter) -> ParseResult {
    match tokens.peek().unwrap() {
        Token::LParen => parse_parens(tokens),

        Token::Number(_) => {
            expect_token!(Token::Number(n) in ITER tokens);
            Ok((Expr::Num(n), true))
        },
        
        Token::Ident(_) => {
            parse_ident(tokens)
        },

        _ => Err(ParseError::UnexpectedToken(tokens.next().unwrap())),
    }
}

fn parse_ident(tokens: &mut TokenIter) -> ParseResult {
    expect_token!(Token::Ident(ident) in ITER tokens);

    if let Some(Token::LParen) = tokens.peek() {
        let (inner, is_const) = parse_parens(tokens)?;
        let func = to_func(ident, inner, is_const);
        return Ok((func, is_const));
    }

    Ok((ident.into(), false))
}

fn parse_parens(tokens: &mut TokenIter) -> ParseResult {
    expect_token!(Token::LParen in ITER tokens);
    let result = parse_expr(tokens);
    println!("{:?}", tokens.peek());
    expect_token_ret!(Token::RParen in ITER tokens);
    result
}

fn to_func(ident: String, inner: Expr, is_const: IsConst) -> Expr {
    let func = wrap_with_func(ident, inner);

    if is_const {
        func.eval_const().into()
    } else {
        func
    }   
}

fn wrap_with_func(ident: String, inner: Expr) -> Expr {
    match ident.as_str() {
        "sin" => Expr::new_sin(inner),
        "cos" => Expr::new_cos(inner),
        _ => panic!("Function name not recognized: {}", ident),
    }
}



fn to_binop(
    token: &Token,
    lhs: Expr,
    rhs: Expr,
    is_const: bool,
) -> Expr {
    let expr = wrap_with_binop(token, lhs, rhs);

    if is_const {
        expr.eval_const().into()
    } else {
        expr
    }
}

fn wrap_with_binop(token: &Token, lhs: Expr, rhs: Expr) -> Expr {
    match token {
        Token::Plus     => Expr::new_add(lhs, rhs),
        Token::Minus    => Expr::new_sub(lhs, rhs),
        Token::Star     => Expr::new_mul(lhs, rhs),
        Token::Slash    => Expr::new_div(lhs, rhs),
        Token::Caret    => Expr::new_pow(lhs, rhs),
        _ => panic!("Unexpected token {:?}", token),
    }
}



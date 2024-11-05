use super::token::{self, Token, TokenIter};
use super::parse_error::ParseError;
use super::macros::expect_token;
use crate::expr::Expr;

type IsConst = bool;
type ParseResult = Result<(Expr, IsConst), ParseError>;

pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    parse_expr(&mut tokens).map(|x| x.0)
}

fn parse_expr(tokens: &mut TokenIter) -> ParseResult {
    let result = parse_sum(tokens);

    // If there are tokens left, the parsing failed
    if let Some(t) = tokens.next() {
        println!("{:?}, {:?}", t, tokens);
        return Err(ParseError::UnexpectedToken(t));
    }

    result
}


// BUG: If i accidentally use the implicit multiplication like so:
// a + bx
// Then the parser will just stop at the 'b' and will not panic neither.
// I have to fix it eventually.
// Or implement implicit multiplication for numbers at beginning. (This may not solve the issue)
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
        let is_both_const = is_lhs_const && is_rhs_const;
        lhs = convert_to_binop(&token, lhs, rhs, is_both_const);
        is_lhs_const = is_both_const;
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
        let func = convert_to_func(ident, inner, is_const);
        return Ok((func, is_const));
    }

    Ok((ident.into(), false))
}
 
fn parse_parens(tokens: &mut TokenIter) -> ParseResult {
    expect_token!(Token::LParen in ITER tokens);
    let result = parse_expr(tokens);
    expect_token!(Token::RParen in ITER tokens);
    result
}

fn convert_to_func(ident: String, inner: Expr, is_const: IsConst) -> Expr {
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


// This merges two expressions into a binary operation expression.
// If both expressions are constant, the operation is evaluated and the result is returned as a constant expression.
// Otherwise, the operation is returned as an expression.
// if the token is not a binary operator, this function panics.
fn convert_to_binop(
    token: &Token,
    lhs: Expr,
    rhs: Expr,
    is_const: bool,
) -> Expr {
    if is_const {
        binop_token_apply_unchecked(token, lhs, rhs).into()
    } else {
        binop_token_wrap(token, lhs, rhs)
    }
}

fn binop_token_apply_unchecked(token: &Token, lhs: Expr, rhs: Expr) -> f32 {
    let lhs = lhs.eval_const();
    let rhs = rhs.eval_const();
    match token {
        Token::Plus  => lhs + rhs,
        Token::Minus => lhs - rhs,
        Token::Star  => lhs * rhs,
        Token::Slash => lhs / rhs,
        Token::Caret => lhs.powf(rhs),
        _ => panic!("Unexpected token {:?}", token),
    }
}

fn binop_token_wrap(token: &Token, lhs: Expr, rhs: Expr) -> Expr {
    match token {
        Token::Plus     => Expr::new_add(lhs, rhs),
        Token::Minus    => Expr::new_sub(lhs, rhs),
        Token::Star     => Expr::new_mul(lhs, rhs),
        Token::Slash    => Expr::new_div(lhs, rhs),
        Token::Caret    => Expr::new_pow(lhs, rhs),
        _ => panic!("Unexpected token {:?}", token),
    }
}



// NOTE: implicit multiplication is not used for now. I don't think that's good idea for plain text
// based lang
//
// Parses expressions without operator between them.
// Only first number as coefficient is parsed.
// It's because there can be umbiguos cases and the parsing of them would be hell more complex.
// So no, I will not implement it.
//fn parse_implicit_multiplication(mut lhs: Expr, mut is_lhs_const: IsConst, tokens: &mut TokenIter) -> ParseResult {
//    while let Some(token) = tokens.peek() {
//        match token {
//            Token::Ident(_) => {
//                expect_token!(Token::Ident(s) in ITER tokens);
//
//                lhs = Expr::new_mul(lhs, Expr::Var(s));
//                is_lhs_const = false;
//            },
//
//            Token::LParen => {
//                let (inner, is_inner_const) = parse_parens(tokens);
//
//                lhs = Expr::new_mul(lhs, inner);
//                is_lhs_const = is_lhs_const && is_inner_const;
//            },
//
//            _ => break,
//        }
//    }
//
//    (lhs, is_lhs_const)
//}


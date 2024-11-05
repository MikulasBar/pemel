use super::token::{self, Token, TokenIter};
use super::parse_error::ParseError;
use super::macros::expect_token;
use crate::expr::Expr;

type IsConst = bool;

pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut tokens = tokens.into_iter().peekable();
    parse_expr(&mut tokens).0  
}

fn parse_expr(tokens: &mut TokenIter) -> (Expr, IsConst) {
    parse_sum(tokens)
}


// BUG: If i accidentally use the implicit multiplication like so:
// a + bx
// Then the parser will just stop at the 'b' and will not panic neither.
// I have to fix it eventually.
// Or implement implicit multiplication for numbers at beginning. (This may not solve the issue)
fn parse_sum(tokens: &mut TokenIter) -> (Expr, IsConst) {
    parse_bin_op(
        |t| matches!(t, Some(Token::Plus | Token::Minus)),
        |iter| parse_product(iter),
        tokens
    )
}

fn parse_product(tokens: &mut TokenIter) -> (Expr, IsConst) {
    parse_bin_op(
        |t| matches!(t, Some(Token::Star | Token::Slash)),
        |iter| parse_power(iter),
        tokens
    )
}

fn parse_power(tokens: &mut TokenIter) -> (Expr, IsConst) {
    parse_bin_op(
        |t| matches!(t, Some(Token::Caret)),
        |iter| parse_atom(iter),
        tokens
    )
}

fn parse_bin_op(
    match_op: fn(Option<&Token>) -> bool,
    parse_prev: fn(&mut TokenIter) -> (Expr, IsConst),
    tokens: &mut TokenIter
) -> (Expr, IsConst) {
    let (mut lhs, mut is_lhs_const) = parse_prev(tokens);

    while match_op(tokens.peek()) {
        expect_token!(token in ITER tokens);

        let (rhs, is_rhs_const) = parse_prev(tokens);
        let is_both_const = is_lhs_const && is_rhs_const;
        lhs = convert_to_bin_op(&token, lhs, rhs, is_both_const);
        is_lhs_const = is_both_const;
    }

    (lhs, is_lhs_const)   
}

fn parse_atom(tokens: &mut TokenIter) -> (Expr, IsConst) {
    let (lhs, is_lhs_const) = match tokens.peek().unwrap() {
        Token::LParen => parse_parens(tokens),

        Token::Number(_) => {
            expect_token!(Token::Number(n) in ITER tokens);
            (Expr::Num(n), true)
        },
        
        Token::Ident(_) => {
            parse_ident(tokens)
        },

        _ => panic!("Unexpected token {:?}", tokens.peek().unwrap()),
    };

    (lhs, is_lhs_const)
}

fn parse_ident(tokens: &mut TokenIter) -> (Expr, IsConst) {
    expect_token!(Token::Ident(ident) in ITER tokens);

    if let Some(Token::LParen) = tokens.peek() {
        let (inner, is_const) = parse_parens(tokens);
        let func = convert_to_func(ident, inner, is_const);
        return (func, is_const);
    }

    (ident.into(), false)
}
 
fn parse_parens(tokens: &mut TokenIter) -> (Expr, IsConst) {
    expect_token!(Token::LParen in ITER tokens);
    let (inner, is_inner_const) = parse_expr(tokens);
    expect_token!(Token::RParen in ITER tokens);
    (inner, is_inner_const)
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
fn convert_to_bin_op(
    token: &Token,
    lhs: Expr,
    rhs: Expr,
    is_const: bool,
) -> Expr {
    if is_const {
        op_token_apply_unchecked(token, lhs, rhs).into()
    } else {
        op_token_wrap(token, lhs, rhs)
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
        Token::Caret => lhs.powf(rhs),
        _ => panic!("Unexpected token"),
    }
}

fn op_token_wrap(token: &Token, lhs: Expr, rhs: Expr) -> Expr {
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
//fn parse_implicit_multiplication(mut lhs: Expr, mut is_lhs_const: IsConst, tokens: &mut TokenIter) -> (Expr, IsConst) {
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


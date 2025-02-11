use crate::eval_error::EvalError;
use crate::expr::Expr;
use crate::parser::ParseError;
use crate::parser::Token;

#[test]
fn bad_syntax() {
    let i1 = "5 + 12x";
    let i2 = "(1 + 5*x) - (45 + x - 2";

    let e1 = Expr::parse(i1, false);
    let e2 = Expr::parse(i2, false);

    let token_x = Token::Ident("x".to_string());

    assert_eq!(e1, Err(ParseError::UnexpectedToken(token_x)));
    assert_eq!(e2, Err(ParseError::UnexpectedToken(Token::EOF)));
}

#[test]
fn unrecognized_function() {
    let input = "sinc(3 * 8)";
    let expr = Expr::parse(input, false);

    assert!(matches!(expr, Err(ParseError::FunctionNotRecognized(_))));
}

#[test]
fn eval_error_in_parse() {
    let input = "2 + 3 / 0";
    let expr = Expr::parse(input, true);

    assert_eq!(expr, Err(ParseError::EvalError(EvalError::DivisionByZero)));
}

// #[test]
// fn wrong_number_of_args() {
//     let input = "cos(2, 4)";
//     let expr = Expr::parse(input, false);

//     assert!(matches!(expr, Err(ParseError::WrongNumberOfArgs(_))));
// }

#[test]
fn const_expr_eval() {
    let input = "8 + 6 * 2.5  + (2 - 2) + 1.5001";
    let expr = Expr::parse(input, true).unwrap();
    let result = expr.eval_const();

    assert_eq!(expr, Expr::Num(24.5001));
    assert_eq!(result, Ok(24.5001));
}

#[test]
fn wrong_const_expr_eval() {
    let input = "2.5*8 + 4*(1 - x)";
    let expr = Expr::parse(input, true).unwrap();
    let result = expr.eval_const();

    assert_eq!(result, Err(EvalError::VariableNotDefined("x".to_string())));
}

#[test]
fn power() {
    let input = "2*x^3 - 3*x^2/2 + 1*x^(2/1) - 5";
    let expr = Expr::parse(input, false).unwrap();

    println!("{}", expr.to_string());

    let zero = expr.eval_with_var("x", 0.0).unwrap();
    let two = expr.eval_with_var("x", 2.0).unwrap();
    let minus_two = expr.eval_with_var("x", -2.0).unwrap();

    assert_eq!(zero, -5.0);
    assert_eq!(two, 9.0);
    assert_eq!(minus_two, -23.0);
}

use std::f32::consts::{E, PI};

#[test]
fn goniometric() {
    let input = "sin(pi/2) + cos(pi) + tan(pi/4) + cot(pi/4)";
    let expr = Expr::parse(input, false).unwrap();
    let result = expr.eval_with_var("pi", PI).unwrap();

    assert!((result - 2.0).abs() <= f32::EPSILON);
}

#[test]
fn log() {
    let input = "log(2, 8) + ln(e^2) - log(100)";
    let expr = Expr::parse(input, true).unwrap();
    let result = expr.eval_with_var("e", E).unwrap();

    assert_eq!(result, 3.0);
}

#[test]
fn eval_with() {
    let input = "2*x + sin(pi) - ln(e)";
    let expr = Expr::parse(input, false).unwrap();

    let result = expr.eval_with(&[
        ("pi", PI),
        ("e", E),
        ("x", 3.0)
    ]).unwrap();

    assert_eq!(result, 5.0);
}

#[test]
fn approx_derivative() {
    let input = "x^2 + 2*x + 1";
    let expr = Expr::parse(input, false).unwrap();
    let result = expr.approx_derivative("x", 2.0, 0.001).unwrap();

    // println!("RESULT: {}", result);
    assert!((result - 6.0).abs() <= 0.0001);
}

#[test]
fn abs() {
    let input = "abs(1) + abs(-1)";
    let expr = Expr::parse(input, true).unwrap();
    let result = expr.eval_const().unwrap();

    assert_eq!(result, 2.0);
}

#[test]
fn derivative_function() {
    let input = "D(x, x^2 + 2*x + 1)";
    let expr = Expr::parse(input, false).unwrap();
    let result = expr.eval_with_var("x", 2.0).unwrap();

    assert!((result - 6.0).abs() <= 0.0001);
}

//////////////////////////////////////////////////////////////////////////////
//  These test can't fail because they are just for testing functionality.  //
//////////////////////////////////////////////////////////////////////////////

#[test]
fn display() {
    let input = "2 + 7 - x + 8 * (x - 1) - 2";
    let expr = Expr::parse(input, true).unwrap();

    print!("{}", expr);
}

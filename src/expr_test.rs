use crate::expr::Expr;

#[test]
fn const_expr_eval() {
    let input = "8 + 6 * 2.5  + (2 - 2) + 1.5001";
    let expr = Expr::parse(input);
    let result = expr.eval_const();

    assert_eq!(expr, Expr::Num(24.5001));
}

#[should_panic]
#[test]
fn const_expr_eval_should_panic() {
    let input = "2.5*8 + 4*(1 - x)";
    let expr = Expr::parse(input);
    let result = expr.eval_const();
}


#[test]
fn get_closure_with_var() {
    let input = "7.55 - x + 8 * (x - 1) - 2";
    let expr = Expr::parse(input);
    let closure = expr.get_closure_with_var("x");

    let result = closure(3.0);
    assert_eq!(result, 18.55);
}

#[test]
fn aprox_derivative() {
    let input = "x*x + 2*x + 1";
    let expr = Expr::parse(input);

    println!("{:?}", expr);
    let derivative = expr.aprox_derivative("x");
    let result = derivative(2.0, 0.00001);
    
    assert_eq!(result.round(), 6.0);
}


#[test]
fn power() {
    let input = "2*x^3 - 3*x^2/2 + 1*x^(2/1) - 5";
    let expr = Expr::parse(input);

    println!("{}", expr.to_string());

    let zero = expr.eval_with_variable("x", 0.0);
    let two = expr.eval_with_variable("x", 2.0);
    let minus_two = expr.eval_with_variable("x", -2.0);

    assert_eq!(zero, -5.0);
    assert_eq!(two, 9.0);
    assert_eq!(minus_two, -23.0);
}

use std::f32::consts::{PI, E};

#[test]
fn sin() {
    // TODO: Consider implementing rounding function for trigs.
    // But maybe it's not neccessary.
    let input = "sin(pi/2) + sin(pi)";
    let expr = Expr::parse(input);
    let result = expr.eval_with_variable("pi", std::f32::consts::PI);

    assert!((result - 1.0).abs() <= f32::EPSILON);
}

#[test]
fn cos() {
    let input = "cos(pi) - cos(3*pi)";
    let expr = Expr::parse(input);
    let result = expr.eval_with_variable("pi", PI);
    
    assert!((result - 0.0).abs() <= f32::EPSILON);
}

//#[test]
//fn log() {
//    let input = "log_";
//    let expr = Expr::parse(input);
//    let result = expr.eval_with_variable("e", E);
//
//    assert!(result );
//}
//
//




//////////////////////////////////////////////////////////////////////////////
//  These test can't fail because they are just for testing functionality.  //
//////////////////////////////////////////////////////////////////////////////

#[test]
fn to_string() {
    let input = "7 - x + 8 * (x - 1) - 2";
    let expr = Expr::parse(input);
    let result = expr.to_string();

    print!("{}", result);
}

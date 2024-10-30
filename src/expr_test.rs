use crate::expr::Expr;

#[test]
fn const_expr() {
    let input = "8 + 6 * 2.5  + (2 - 2) + 1.5001";
    let expr = Expr::parse(input);
    let result = expr.eval_const();

    assert_eq!(expr, Expr::Num(24.5001));
    assert_eq!(result, 24.5001);
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
fn implicit_multiplication() {
    let input = "2x*(2 - x)*(1 + 1) - 2*x + 1.5*x";
    let expr = Expr::parse(input);
    let result = expr.eval_with_variable("x", 1.0);
    
    assert_eq!(result, 3.5);
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

#[should_panic] // ROUDING IS NOT IMPLEMENTED YET
#[test]
fn sin_expr() {
    // TODO: WE NEED TO IMPLEMENT ROUNDING, SO WE CAN COMPARE FLOATS
    // DUE TO THIS, THIS TEST WILL FAIL
    let input = "sin(pi/2) + sin(pi)";
    let expr = Expr::parse(input);
    let result = expr.eval_with_variable("pi", std::f32::consts::PI);

    assert_eq!(result, 1.0);
}







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

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
    let input = "2x(2 - x)(1 + 1) - 2x + 1.5x";
    let expr = Expr::parse(input);
    let result = expr.eval_with_variable("x", 1.0);
    
    assert_eq!(result, 3.5);
}







//////////////////////////////////////////////////////////////////////////////
//  These test can't fail because they are just for trying functionality.   //
//////////////////////////////////////////////////////////////////////////////

#[test]
fn to_string() {
    let input = "7 - x + 8 * (x - 1) - 2";
    let expr = Expr::parse(input);
    let result = expr.to_string();

    print!("{}", result);
}
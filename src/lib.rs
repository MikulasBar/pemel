mod expr;
mod parser;
mod eval_error;
mod macros;

use expr::Expr;

#[test]
fn testing() {
    let input = " 8 + 9 * 2  + (2 - x) + 1";
    let expr = Expr::parse(input);
    println!("{:?}", expr);
    // let result = expr.eval_with_variable("x", 9);
    // assert_eq!(result, 20);
}

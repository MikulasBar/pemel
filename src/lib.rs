mod expr;
mod parser;
mod eval_error;
mod macros;

#[test]
fn testing() {
    let input = " 8 + 9 * 2  + (2 - 9) + 1";
    let expr = parser::parse(input);
    let result = expr.eval_const();
    assert_eq!(result, 20);
}
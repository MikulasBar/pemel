# Changelog

All notable changes to this project will be documented in this file.

## 0.1.0 - 2024-01-22

### Added
- Initial release of PEMEL (Parsing and Evaluating of Mathematical Expressions Library).
- Support for parsing basic mathematical expressions.
- Support for evaluating parsed expressions.
- Error handling for syntax errors and evaluation errors.
- Basic arithmetic, logarithms, exponents, and trigonometric functions.
- Unit tests for core functionality.


## 0.2.0 - 2024-02-05

### Added
- Support for tangent and cotangent functions.
- Evaluation with multiple variables.
- Absolute value function.
- Support for prefix unary operators ( +a, -a).
- toggleable implicit evaluation during parsing.
- function like derivative D(x, ...).

### Changed
- Changed approx_derivative() function to return a value instead of a function.

### Removed
- get_closure() function.
- disabled WrongNumberOfArguments exception.

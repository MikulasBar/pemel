# PEMEL

[![Crates.io](https://img.shields.io/crates/v/pemel.svg)](https://crates.io/crates/pemel)
[![License](https://img.shields.io/crates/l/pemel.svg)](https://crates.io/crates/pemel)

## Overview

`pemel` is a Rust library providing a utilities for parsing and evaluating mathematical expressions.

## Features

- Basic arithmetic operations
- Trigonometric functions
- Exponential and logarithmic functions
- Absolute value function
- Evaluation with multiple variables
- Numeric derivatives
- Implicit evaluation during parsing
- Substitution

## Usage

Here is a simple example of how to use `pemel`:

```rust
use pemel::prelude::*;

fn main() {
    let input = "2 * x^2 - 5 * log(x)";
    let expr = Expr::parse(input, true).unwrap();
    let result = expr.eval_with_var("x", 10.0).unwrap();

    println!("{}", result); // Output: 195.0
}
```

## Contributing

Untill I create a `CONTRIBUTING.md` file, I will not accept any pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
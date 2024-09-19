use super::token::Token;
use super::macros::char_pat;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut chars = input.chars().peekable();

    while let Some(&char) = chars.peek() {
        let token = match char {
            ' ' => {
                chars.next();
                continue;
            },

            '+' => {
                chars.next();
                Token::Plus
            },

            '-' => {
                chars.next();
                Token::Minus
            },

            '*' => {
                chars.next();
                Token::Star
            },

            '/' => {
                chars.next();
                Token::Slash
            },

            '(' => {
                chars.next();
                Token::LParen
            },

            ')' => {
                chars.next();
                Token::RParen
            },

            '0'..='9' => {
                let mut num_str = String::new();
                parse_digits(&mut num_str, &mut chars);
                
                if chars.peek() == Some(&'.') {
                    chars.next();
                    num_str.push('.');
                    parse_digits(&mut num_str, &mut chars);

                    if num_str.chars().last() == Some('.') {
                        panic!("Number literal must have digits after the decimal point");
                    }
                }

                Token::Number(num_str.parse().unwrap())
            },

            char_pat!(IDENT) => {
                let mut ident_str = String::new();

                while let Some(&char) = chars.peek() {
                    match char {
                        char_pat!(IDENT) => {
                            ident_str.push(char);
                            chars.next();
                        },
                        _ => break,
                    }
                }

                Token::Ident(ident_str)
            },

            _ => panic!(),
        };

        tokens.push(token);
    }

    tokens
}


fn parse_digits(num_str: &mut String, chars: &mut std::iter::Peekable<std::str::Chars>) {
    while let Some(&char) = chars.peek() {
        match char {
            '0'..='9' => {
                num_str.push(char);
                chars.next();
            },
            _ => break,
        }
    }
}
use super::token::Token;

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedChar(char),
    WrongNumberOfArgs(usize),
    FunctionNotRecognized(String),
}

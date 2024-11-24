use std::iter::Peekable;
use std::vec::IntoIter;

pub type TokenIter = Peekable<IntoIter<Token>>;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Number(f32),
    // sin, and other special names are also Ident
    Ident(String),
    //Punctuated(Punctation, Vec<Box<Token>>),
    Comma,
    EOF,
}
//
//#[derive(Debug, PartialEq, Clone)]
//pub enum Punctation {
//    Comma,
//}

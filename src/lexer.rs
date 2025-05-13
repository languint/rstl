use std::fmt;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Literal,
    #[regex(r"-?\d+(\.\d+)?")]
    NumberLiteral,
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,
    #[token("mut")]
    MutKeyword,
    #[token("const")]
    ConstKeyword,
    // Delimiters
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(",")]
    Comma,
    #[token(".")]
    Period,
    // Operators
    #[token("=")]
    Equals,
    #[token("==")]
    BoolEquals,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self)
    }
}

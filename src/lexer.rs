use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Literal,
    #[regex(r"\d+(\.\d+)?")]
    NumberLiteral,
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,
    // Chars
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    // Keywords
    #[token("mut")]
    MutKeyword,
    #[token("const")]
    ConstKeyword,
    #[token("fn")]
    FunctionKeyword,
    #[token("return")]
    ReturnKeyword,
    // Math Operators
    #[token("=")]
    Equals,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    // Operators
    #[token("==")]
    DoubleEquals,
    #[token("!=")]
    NotEquals,
    #[token("->")] // Really only for function return type annotations.
    To,
    // Types
    #[token("string")]
    StringType,
    #[token("float")]
    FloatType,
    #[token("int")]
    IntType,
    #[token("bool")]
    BoolType,
}

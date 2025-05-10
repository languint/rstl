#[cfg(test)]
pub mod lexer_test {
    use crate::tests::test_token_types;

    #[test]
    pub fn lexer_integer() {
        use crate::lexer::Token;
        use logos::Logos;

        let file_content = r"1234567890";

        let parser = Token::lexer(file_content);
        let tokens: Vec<_> = parser.collect();

        assert_eq!(tokens.len(), 1, "Incorrect number of tokens!");
        assert_eq!(
            tokens[0],
            Ok(Token::NumberLiteral),
            "Outputted token is not a NumberLiteral!"
        );
    }

    #[test]
    pub fn lexer_decimal() {
        use crate::lexer::Token;
        use logos::Logos;

        let file_content = r"1234567890.1234567890";

        let parser = Token::lexer(file_content);
        let tokens: Vec<_> = parser.collect();

        assert_eq!(tokens.len(), 1, "Incorrect number of tokens!");
        assert_eq!(
            tokens[0],
            Ok(Token::NumberLiteral),
            "Outputted token is not a NumberLiteral!"
        );
    }

    #[test]
    pub fn lexer_string() {
        use crate::lexer::Token;
        use logos::Logos;

        let file_content = r#"
            "string0123456789"
            "string"
        "#;

        let parser = Token::lexer(file_content);
        let tokens: Vec<_> = parser.collect();

        assert_eq!(tokens.len(), 2, "Incorrect number of tokens!");
        test_token_types(tokens, vec![Token::StringLiteral, Token::StringLiteral]);
    }

    #[test]
    pub fn lexer_literals() {
        use logos::Logos;

        use crate::lexer::Token;

        let file_content = r"my_variable";

        let parser = Token::lexer(file_content);

        let tokens: Vec<_> = parser.collect();

        assert_eq!(tokens.len(), 1, "Too many tokens!");
        assert_eq!(
            tokens[0],
            Ok(Token::Literal),
            "Outputted token is not a Literal!"
        );
    }

    #[test]
    pub fn lexer_variable() {
        use logos::Logos;

        use crate::lexer::Token;

        let file_content = r"
            mut a = 10;
            const b = 20;
        ";

        let parser = Token::lexer(file_content);

        let tokens: Vec<_> = parser.collect();

        assert_eq!(tokens.len(), 10, "Incorrect number of tokens!");
        test_token_types(
            tokens,
            vec![
                Token::MutKeyword,
                Token::Literal,
                Token::Equals,
                Token::NumberLiteral,
                Token::Semicolon,
                Token::ConstKeyword,
                Token::Literal,
                Token::Equals,
                Token::NumberLiteral,
                Token::Semicolon,
            ],
        );
    }

    #[test]
    pub fn lexer_function() {
        use logos::Logos;

        use crate::lexer::Token;

        let file_content = r"
            fn zero() -> float {
                return 0.0;
            }
        ";

        let parser = Token::lexer(file_content);

        let tokens: Vec<_> = parser.collect();

        assert_eq!(tokens.len(), 11, "Incorrect number of tokens!");
        test_token_types(
            tokens,
            vec![
                Token::FunctionKeyword,
                Token::Literal,
                Token::LeftParen,
                Token::RightParen,
                Token::To,
                Token::Literal,
                Token::LeftBrace,
                Token::ReturnKeyword,
                Token::NumberLiteral,
                Token::Semicolon,
                Token::RightBrace,
            ],
        );
    }
}

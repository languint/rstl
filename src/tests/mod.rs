use crate::{lexer::Token, syntax_tree::AstNode};

pub mod lexer_test;
pub mod syntax_tree_test;

pub fn test_token_types(tokens: Vec<Result<Token, ()>>, expected_types: Vec<Token>) {
    for (i, token) in tokens.into_iter().enumerate() {
        if token.is_err() {
            panic!("Token at {i} is Err")
        }
        assert_eq!(token.unwrap(), expected_types[i], "Mismatch at token {}", i);
    }
}

pub fn test_node_types(tokens: Vec<AstNode>, expected_types: Vec<AstNode>) {
    for (i, token) in tokens.into_iter().enumerate() {
        assert_eq!(token, expected_types[i], "Mismatch at node {}", i);
    }
}

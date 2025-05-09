#[cfg(test)]
pub mod syntax_tree_test {
    use crate::{
        syntax_tree::{AstNode, SyntaxTree},
        tests::test_node_types,
    };

    #[test]
    pub fn syntax_tree_variable_math() {
        use crate::lexer::Token;
        use logos::Logos;

        let file_content = r"
            mut a = 10;
            const b = 20;
            const c = a + b;
        ";

        let lexer = Token::lexer(file_content);
        let nodes = SyntaxTree::new(lexer).build();

        assert!(nodes.is_ok(), "An error occurred with building the AST");

        test_node_types(
            nodes.unwrap(),
            vec![
                AstNode::VarDeclaration {
                    mutable: true,
                    name: "a".to_string(),
                    value: Box::from(AstNode::Number(10.0)),
                },
                AstNode::VarDeclaration {
                    mutable: false,
                    name: "b".to_string(),
                    value: Box::from(AstNode::Number(20.0)),
                },
                AstNode::VarDeclaration {
                    mutable: false,
                    name: "c".to_string(),
                    value: Box::from(AstNode::BinaryOp {
                        left: Box::from(AstNode::Identifier("a".to_string())),
                        op: Token::Plus,
                        right: Box::from(AstNode::Identifier("b".to_string())),
                    }),
                },
            ],
        );
    }

    #[test]
    pub fn syntax_tree_math_parens() {
        use crate::lexer::Token;
        use logos::Logos;

        let file_content = r"
            mut a = 10;
            const b = 20;
            const c = 2 * (a + b);
        ";

        let lexer = Token::lexer(file_content);
        let nodes = SyntaxTree::new(lexer).build();

        assert!(nodes.is_ok(), "An error occurred with building the AST");

        test_node_types(
            nodes.unwrap(),
            vec![
                AstNode::VarDeclaration {
                    mutable: true,
                    name: "a".to_string(),
                    value: Box::from(AstNode::Number(10.0)),
                },
                AstNode::VarDeclaration {
                    mutable: false,
                    name: "b".to_string(),
                    value: Box::from(AstNode::Number(20.0)),
                },
                AstNode::VarDeclaration {
                    mutable: false,
                    name: "c".to_string(),
                    value: Box::from(AstNode::BinaryOp {
                        left: Box::from(AstNode::Number(2.0)),
                        op: Token::Star,
                        right: Box::from(AstNode::BinaryOp {
                            left: Box::from(AstNode::Identifier("a".to_string())),
                            op: Token::Plus,
                            right: Box::from(AstNode::Identifier("b".to_string())),
                        }),
                    }),
                },
            ],
        );
    }
}

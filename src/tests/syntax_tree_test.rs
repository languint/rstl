#[cfg(test)]
pub mod syntax_tree_test {
    use crate::{
        syntax_tree::{AstNode, SyntaxTree, TypeAnnotation},
        tests::test_node_types,
    };

    #[test]
    pub fn syntax_tree_variable_math() {
        use crate::lexer::Token;
        use logos::Logos;

        let file_content = r"
            mut a~int = 10;
            const b~float = 20.0;
            const c~int = a + b;
        ";

        let lexer = Token::lexer(file_content);
        let nodes = SyntaxTree::new(lexer).build();

        assert!(
            nodes.is_ok(),
            "An error occurred with building the AST: `{}`",
            nodes.unwrap_err()
        );

        test_node_types(
            nodes.unwrap(),
            vec![
                AstNode::VarDeclaration {
                    mutable: true,
                    name: "a".to_string(),
                    value: Box::from(AstNode::Number(10.0)),
                    type_annotation: Option::from(TypeAnnotation::Int),
                },
                AstNode::VarDeclaration {
                    mutable: false,
                    name: "b".to_string(),
                    value: Box::from(AstNode::Number(20.0)),
                    type_annotation: Option::from(TypeAnnotation::Float),
                },
                AstNode::VarDeclaration {
                    mutable: false,
                    name: "c".to_string(),
                    value: Box::from(AstNode::BinaryOp {
                        left: Box::from(AstNode::Identifier("a".to_string())),
                        op: Token::Plus,
                        right: Box::from(AstNode::Identifier("b".to_string())),
                    }),
                    type_annotation: Option::from(TypeAnnotation::Int),
                },
            ],
        );
    }

    #[test]
    pub fn syntax_tree_math_parens() {
        use crate::lexer::Token;
        use logos::Logos;

        let file_content = r"
            mut a~int = 10;
            const b~float = 20.0;
            const c~float = 2 * (a + b);
        ";

        let lexer = Token::lexer(file_content);
        let nodes = SyntaxTree::new(lexer).build();

        assert!(
            nodes.is_ok(),
            "An error occurred with building the AST: `{}`",
            nodes.unwrap_err()
        );

        test_node_types(
            nodes.unwrap(),
            vec![
                AstNode::VarDeclaration {
                    mutable: true,
                    name: "a".to_string(),
                    value: Box::from(AstNode::Number(10.0)),
                    type_annotation: Option::from(TypeAnnotation::Int),
                },
                AstNode::VarDeclaration {
                    mutable: false,
                    name: "b".to_string(),
                    value: Box::from(AstNode::Number(20.0)),
                    type_annotation: Option::from(TypeAnnotation::Float),
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
                    type_annotation: Option::from(TypeAnnotation::Float),
                },
            ],
        );
    }

    #[test]
    pub fn syntax_tree_function() {
        use crate::lexer::Token;
        use logos::Logos;

        let file_content = r"
            fn add(a ~ int, b ~ int) -> int {
                return a + b;
            }

            fn sub(a ~ int, b ~ int) -> int {
                return a - b;
            }

            add(2, 4);
        ";

        let lexer = Token::lexer(file_content);
        let nodes = SyntaxTree::new(lexer).build();

        assert!(
            nodes.is_ok(),
            "An error occurred with building the AST: `{}`",
            nodes.unwrap_err()
        );

        test_node_types(
            nodes.unwrap(),
            vec![
                AstNode::FnDeclaration {
                    name: "add".to_string(),
                    args: vec![
                        AstNode::FnArgument {
                            name: "a".to_string(),
                            arg_type: TypeAnnotation::Int,
                        },
                        AstNode::FnArgument {
                            name: "b".to_string(),
                            arg_type: TypeAnnotation::Int,
                        },
                    ],
                    return_type: Option::from(TypeAnnotation::Int),
                    body: vec![AstNode::Return(Box::from(AstNode::BinaryOp {
                        left: Box::from(AstNode::Identifier("a".to_string())),
                        op: Token::Plus,
                        right: Box::from(AstNode::Identifier("b".to_string())),
                    }))],
                },
                AstNode::FnDeclaration {
                    name: "sub".to_string(),
                    args: vec![
                        AstNode::FnArgument {
                            name: "a".to_string(),
                            arg_type: TypeAnnotation::Int,
                        },
                        AstNode::FnArgument {
                            name: "b".to_string(),
                            arg_type: TypeAnnotation::Int,
                        },
                    ],
                    return_type: Option::from(TypeAnnotation::Int),
                    body: vec![AstNode::Return(Box::from(AstNode::BinaryOp {
                        left: Box::from(AstNode::Identifier("a".to_string())),
                        op: Token::Minus,
                        right: Box::from(AstNode::Identifier("b".to_string())),
                    }))],
                },
                AstNode::FnCall {
                    name: "add".to_string(),
                    args: vec![AstNode::Number(2.0), AstNode::Number(4.0)],
                },
            ],
        );
    }
}

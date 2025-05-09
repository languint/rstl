use logos::Lexer;

use crate::{lexer::Token, util::print_error};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    Int,
    Float,
    Bool,
    String,
    Custom(String), // For user defined types
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    // Literals, Identifiers.
    Number(f64),
    String(String),
    Identifier(String),

    // Declarations & Assignments
    VarDeclaration {
        mutable: bool,
        name: String,
        value: Box<AstNode>,
    },
    VarAssignment {
        name: String,
        value: Box<AstNode>,
    },

    // Return
    Return(Box<AstNode>),

    // Binary Operations
    BinaryOp {
        left: Box<AstNode>,
        op: Token,
        right: Box<AstNode>,
    },

    // Functions
    FnArgument {
        name: String,
        arg_type: TypeAnnotation,
    },
    FnDeclaration {
        name: String,
        args: Vec<AstNode>,
        return_type: Option<TypeAnnotation>,
        body: Vec<AstNode>,
    },
    FnCall {
        name: String,
        args: Vec<AstNode>,
    },
}

pub struct SyntaxTree {
    lexer_tokens: Vec<(Token, String)>,
}

impl SyntaxTree {
    pub fn new(mut lexer: Lexer<'static, Token>) -> Self {
        let mut lexer_tokens = Vec::new();

        while let Some(tok) = lexer.next() {
            match tok {
                Ok(t) => {
                    let text = lexer.slice().to_string();
                    lexer_tokens.push((t, text));
                }
                Err(_) => print_error(format!("Invalid Token {:?}", lexer.slice()).as_str(), 0),
            }
        }

        Self { lexer_tokens }
    }

    #[inline]
    fn peek(&self, i: usize) -> Option<&Token> {
        self.lexer_tokens.get(i).map(|(tok, _)| tok)
    }

    pub fn build(&self) -> Result<Vec<AstNode>, String> {
        let mut nodes = Vec::new();
        let mut i = 0;
        let tokens = &self.lexer_tokens;

        while let Some(tok) = self.peek(i) {
            match tok {
                Token::MutKeyword | Token::ConstKeyword => {
                    let mutable = *tok == Token::MutKeyword;
                    i += 1; // eat `mut` or `const`

                    let name = if let Some(Token::Literal) = self.peek(i) {
                        let s = tokens[i].1.clone();
                        i += 1; // consume literal

                        s
                    } else {
                        return Err(format!("Expected variable name at {}", i));
                    };

                    if self.peek(i) != Some(&Token::Equals) {
                        return Err(format!("Expected `=` after var name at {}", i));
                    }

                    i += 1; // consume =

                    // parse an expression as the initializer
                    let expr = self.parse_expression(&mut i);

                    if expr.is_err() {
                        return Err(format!("{}", expr.err().unwrap()));
                    }

                    if self.peek(i) != Some(&Token::Semicolon) {
                        return Err(String::from("Expected `;` at end of var decl"));
                    }

                    i += 1; // consume ;

                    let expr = expr.unwrap();

                    nodes.push(AstNode::VarDeclaration {
                        mutable,
                        name,
                        value: Box::new(expr),
                    });
                }

                Token::ReturnKeyword => {
                    i += 1; // consume `return`

                    let expr = self.parse_expression(&mut i);

                    if expr.is_err() {
                        return Err(format!("{}", expr.err().unwrap()));
                    }

                    let expr = expr.unwrap();

                    if self.peek(i) == Some(&Token::Semicolon) {
                        i += 1;
                    }

                    nodes.push(AstNode::Return(Box::new(expr)));
                }

                _ => {
                    let expr = self.parse_expression(&mut i);

                    if expr.is_err() {
                        return Err(format!("{}", expr.err().unwrap()));
                    }

                    let expr = expr.unwrap();

                    match expr {
                        AstNode::BinaryOp { left, op, right } => {
                            if let AstNode::Identifier(name) = *left {
                                nodes.push(AstNode::VarAssignment { name, value: right });

                                if self.peek(i) == Some(&Token::Semicolon) {
                                    i += 1;
                                }

                                continue;
                            }

                            let expr = AstNode::BinaryOp { left, op, right };
                            nodes.push(expr);
                        }

                        other => {
                            nodes.push(other);
                        }
                    }
                }
            }
        }

        Ok(nodes)
    }

    fn parse_expression(&self, i: &mut usize) -> Result<AstNode, String> {
        let node = match self.peek(*i) {
            Some(Token::NumberLiteral) => {
                let text = &self.lexer_tokens[*i].1;

                *i += 1;

                let val = text.parse::<f64>();

                if val.is_err() {
                    return Err(format!("{}", val.err().unwrap()));
                }

                Ok(AstNode::Number(val.unwrap()))
            }
            Some(Token::StringLiteral) => {
                let raw = &self.lexer_tokens[*i].1;

                *i += 1;

                let inner = &raw[1..raw.len() - 1];

                Ok(AstNode::String(inner.replace("\\\"", "\"")))
            }
            Some(Token::Literal) => {
                // identifier or function call
                let name = self.lexer_tokens[*i].1.clone();
                *i += 1;

                // if next is '(', parse as call
                if self.peek(*i) == Some(&Token::LeftParen) {
                    *i += 1; // consume '('
                    let mut args = Vec::new();
                    while self.peek(*i) != Some(&Token::RightParen) {
                        let arg = self.parse_expression(i);
                        if arg.is_err() {
                            return Err(format!("{}", arg.err().unwrap()));
                        }
                        args.push(arg.unwrap());
                        if self.peek(*i) == Some(&Token::Comma) {
                            *i += 1;
                        }
                    }

                    *i += 1; // consume ')'
                    Ok(AstNode::FnCall { name, args })
                } else {
                    Ok(AstNode::Identifier(name))
                }
            }
            Some(Token::LeftParen) => {
                *i += 1;
                let expr = self.parse_expression(i);
                assert_eq!(
                    self.peek(*i),
                    Some(&Token::RightParen),
                    "Expected `)` at {}",
                    i
                );
                *i += 1;
                expr
            }
            _ => Err(String::from("Unexpected expression at {i}")),
        };

        if node.is_err() {
            return Err(format!("{}", node.err().unwrap()));
        }

        let mut node = node.unwrap();

        while let Some(op) = self.peek(*i) {
            let is_binop = matches!(
                op,
                Token::Plus | Token::Minus | Token::DoubleEquals | Token::NotEquals | Token::Star
            );

            if !is_binop {
                break;
            }

            let op_tok = op.clone();
            *i += 1;
            // parse the right-hand side
            let right = self.parse_expression(i);

            if right.is_err() {
                return Err(format!("{}", right.err().unwrap()));
            }

            let right = right.unwrap();

            let left = Box::new(node);

            node = AstNode::BinaryOp {
                left,
                op: op_tok,
                right: Box::new(right),
            };
        }

        Ok(node)
    }
}

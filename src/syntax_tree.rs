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
        type_annotation: Option<TypeAnnotation>, // Added type annotation field
    },
    VarAssignment {
        name: String,
        value: Box<AstNode>,
    },

    // Type Annotation Node
    TypeAnnotationNode(TypeAnnotation),

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

    // Function to parse type annotations
    fn parse_type_annotation(&self, i: &mut usize) -> Result<TypeAnnotation, String> {
        if let Some(Token::Literal) = self.peek(*i) {
            let type_name = self.lexer_tokens[*i].1.clone();
            *i += 1; // consume the type name

            match type_name.as_str() {
                "int" => Ok(TypeAnnotation::Int),
                "float" => Ok(TypeAnnotation::Float),
                "bool" => Ok(TypeAnnotation::Bool),
                "string" => Ok(TypeAnnotation::String),
                _ => Ok(TypeAnnotation::Custom(type_name)),
            }
        } else {
            Err(format!("Expected type annotation at {}", *i))
        }
    }

    // Parse function arguments
    fn parse_function_args(&self, i: &mut usize) -> Result<Vec<AstNode>, String> {
        let mut args = Vec::new();

        // Check if there are any arguments (right parenthesis immediately)
        if self.peek(*i) == Some(&Token::RightParen) {
            *i += 1; // consume ')'
            return Ok(args);
        }

        // Parse arguments
        loop {
            // Get argument name
            if let Some(Token::Literal) = self.peek(*i) {
                let arg_name = self.lexer_tokens[*i].1.clone();
                *i += 1; // consume argument name

                // Check for type annotation
                if self.peek(*i) != Some(&Token::Tilde) {
                    return Err("Expected '~' after argument name".to_string());
                }
                *i += 1; // consume '~'

                // Parse argument type
                let arg_type = self.parse_type_annotation(i)?;

                args.push(AstNode::FnArgument {
                    name: arg_name,
                    arg_type,
                });

                // Check for comma or closing parenthesis
                match self.peek(*i) {
                    Some(Token::Comma) => {
                        *i += 1; // consume ','
                    }
                    Some(Token::RightParen) => {
                        *i += 1; // consume ')'
                        break;
                    }
                    _ => return Err("Expected ',' or ')' after function argument".to_string()),
                }
            } else {
                return Err("Expected argument name".to_string());
            }
        }

        Ok(args)
    }

    // Parse function body
    fn parse_function_body(&self, i: &mut usize) -> Result<Vec<AstNode>, String> {
        if self.peek(*i) != Some(&Token::LeftBrace) {
            return Err("Expected '{' to start function body".to_string());
        }
        *i += 1; // consume '{'

        let mut body = Vec::new();

        while self.peek(*i) != Some(&Token::RightBrace) {
            if *i >= self.lexer_tokens.len() {
                return Err("Unexpected end of input while parsing function body".to_string());
            }

            // Parse statements in the function body
            match self.peek(*i) {
                Some(Token::MutKeyword) | Some(Token::ConstKeyword) => {
                    // Parse variable declaration
                    let mutable = self.peek(*i) == Some(&Token::MutKeyword);
                    *i += 1; // eat `mut` or `const`

                    let name = if let Some(Token::Literal) = self.peek(*i) {
                        let s = self.lexer_tokens[*i].1.clone();
                        *i += 1; // consume literal
                        s
                    } else {
                        return Err(format!("Expected variable name at {}", i));
                    };

                    // Check for type annotation
                    let mut type_annotation = None;
                    if self.peek(*i) == Some(&Token::Tilde) {
                        *i += 1; // consume '~'
                        let type_result = self.parse_type_annotation(i);
                        if type_result.is_err() {
                            return Err(type_result.err().unwrap());
                        }
                        type_annotation = Some(type_result.unwrap());
                    }

                    if self.peek(*i) != Some(&Token::Equals) {
                        return Err(format!("Expected `=` after var name at {}", i));
                    }

                    *i += 1; // consume =

                    // parse an expression as the initializer
                    let expr = self.parse_expression(i)?;

                    if self.peek(*i) != Some(&Token::Semicolon) {
                        return Err(String::from("Expected `;` at end of var decl"));
                    }

                    *i += 1; // consume ;

                    body.push(AstNode::VarDeclaration {
                        mutable,
                        name,
                        value: Box::new(expr),
                        type_annotation,
                    });
                }
                Some(Token::ReturnKeyword) => {
                    *i += 1; // consume `return`

                    let expr = self.parse_expression(i)?;

                    if self.peek(*i) != Some(&Token::Semicolon) {
                        return Err(String::from("Expected `;` after return statement"));
                    }
                    *i += 1; // consume ';'

                    body.push(AstNode::Return(Box::new(expr)));
                }
                _ => {
                    // Parse expressions
                    let expr = self.parse_expression(i)?;

                    match expr {
                        AstNode::BinaryOp { left, op, right } => {
                            if let AstNode::Identifier(name) = *left {
                                if op == Token::Equals {
                                    body.push(AstNode::VarAssignment { name, value: right });
                                } else {
                                    body.push(AstNode::BinaryOp {
                                        left: Box::new(AstNode::Identifier(name)),
                                        op,
                                        right,
                                    });
                                }
                            } else {
                                body.push(AstNode::BinaryOp { left, op, right });
                            }
                        }
                        other => {
                            body.push(other);
                        }
                    }

                    // Expect semicolon after expressions
                    if self.peek(*i) == Some(&Token::Semicolon) {
                        *i += 1; // consume ';'
                    } else {
                        return Err(String::from(
                            "Expected `;` after expression in function body",
                        ));
                    }
                }
            }
        }

        // Consume the closing brace
        *i += 1; // consume '}'

        Ok(body)
    }

    // Parse function declaration
    fn parse_function_declaration(&self, i: &mut usize) -> Result<AstNode, String> {
        *i += 1; // consume 'fn'

        let name = if let Some(Token::Literal) = self.peek(*i) {
            let s = self.lexer_tokens[*i].1.clone();
            *i += 1; // consume function name
            s
        } else {
            return Err("Expected function name after 'fn'".to_string());
        };

        if self.peek(*i) != Some(&Token::LeftParen) {
            return Err("Expected '(' after function name".to_string());
        }

        *i += 1; // consume '('

        let args = self.parse_function_args(i)?;

        let mut return_type = None;
        if self.peek(*i) == Some(&Token::To) {
            *i += 1; // consume '->'
            return_type = Some(self.parse_type_annotation(i)?);
        }

        let body = self.parse_function_body(i)?;

        Ok(AstNode::FnDeclaration {
            name,
            args,
            return_type,
            body,
        })
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

                    // Check for type annotation
                    let mut type_annotation = None;
                    if self.peek(i) == Some(&Token::Tilde) {
                        i += 1; // consume '~'
                        let type_result = self.parse_type_annotation(&mut i);
                        if type_result.is_err() {
                            return Err(type_result.err().unwrap());
                        }
                        type_annotation = Some(type_result.unwrap());
                    }

                    if self.peek(i) != Some(&Token::Equals) {
                        return Err(format!("Expected `=` after var name at {}", i));
                    }

                    i += 1; // consume =

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
                        type_annotation,
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

                Token::FunctionKeyword => {
                    // Parse function declaration
                    let fn_decl = self.parse_function_declaration(&mut i)?;
                    nodes.push(fn_decl);
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
                                if op == Token::Equals {
                                    nodes.push(AstNode::VarAssignment { name, value: right });
                                } else {
                                    nodes.push(AstNode::BinaryOp {
                                        left: Box::new(AstNode::Identifier(name)),
                                        op,
                                        right,
                                    });
                                }

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

                    // Expect semicolon after expressions at top level
                    if self.peek(i) == Some(&Token::Semicolon) {
                        i += 1; // consume ';'
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
                let name = self.lexer_tokens[*i].1.clone();
                *i += 1;

                // if next is '(', parse as call
                if self.peek(*i) == Some(&Token::LeftParen) {
                    *i += 1; // consume '('
                    let mut args = Vec::new();

                    // Parse function call arguments
                    if self.peek(*i) != Some(&Token::RightParen) {
                        loop {
                            let arg = self.parse_expression(i)?;
                            args.push(arg);

                            match self.peek(*i) {
                                Some(Token::Comma) => {
                                    *i += 1; // consume ','
                                }
                                Some(Token::RightParen) => {
                                    break;
                                }
                                _ => {
                                    return Err("Expected ',' or ')' in function call arguments"
                                        .to_string());
                                }
                            }
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
                let expr = self.parse_expression(i)?;

                if self.peek(*i) != Some(&Token::RightParen) {
                    return Err(format!("Expected `)` at {}", i));
                }
                *i += 1;
                Ok(expr)
            }
            _ => Err(String::from(format!(
                "Unexpected expression at `{}`, `{}` ",
                i,
                self.lexer_tokens[*i].1.clone()
            ))),
        };

        if node.is_err() {
            return Err(format!("{}", node.err().unwrap()));
        }

        let mut node = node.unwrap();

        while let Some(op) = self.peek(*i) {
            // Handle assignment operator
            if op == &Token::Equals {
                if let AstNode::Identifier(name) = node {
                    *i += 1; // consume '='
                    let right = self.parse_expression(i)?;
                    return Ok(AstNode::BinaryOp {
                        left: Box::new(AstNode::Identifier(name)),
                        op: Token::Equals,
                        right: Box::new(right),
                    });
                }
            }

            // Handle other binary operators
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
            let right = self.parse_expression(i)?;
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

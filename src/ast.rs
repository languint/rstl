use logos::Lexer;

use crate::{errors::Errors, lexer::Token};

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    Variable(String),
    BinaryOp {
        op: BinaryOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum Statement {
    ExpressionStatement(Expression),
    VarDeclaration {
        name: String,
        ty: Type,
        initializer: Option<Expression>,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Statement>,
    },
    Return(Option<Expression>),
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    Void,
}

pub struct AST {
    tokens: Vec<(Token, String)>,
}

impl AST {
    pub fn new(mut lexer: Lexer<'static, Token>) -> Result<Self, Errors> {
        let mut tokens = Vec::new();

        while let Some(tok) = lexer.next() {
            match tok {
                Ok(t) => {
                    let text = lexer.slice().to_string();
                    tokens.push((t, text));
                }
                Err(_) => return Err(Errors::InvalidToken(format!("{}", lexer.slice()))),
            }
        }

        return Ok(AST { tokens });
    }
}

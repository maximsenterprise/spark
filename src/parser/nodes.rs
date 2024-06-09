// nodes.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use std::error;

use crate::{error, lexer::tokens::{Token, TokenType}};

#[derive(Debug)]
pub struct StringLiteral {
    pub value: String,
    pub token: Token
}

#[derive(Debug)]
pub struct Metadata {
    pub value: Vec<Token>,
    pub token: Token,
}

#[derive(Debug)] 
pub struct Style {
    pub value: String,
    pub token: Token,
}

#[derive(Debug)]
pub struct NumericLiteral {
    pub value: f64,
    pub token: Token,
}

#[derive(Debug)]
pub struct BooleanLiteral {
    pub value: bool,
    pub token: Token,
}

#[derive(Debug)]
pub struct Literal {
    pub value: String,
    pub token: Token,
}

#[derive(Debug)]
pub struct Other {
    pub value: String,
    pub token: Token,
}

#[derive(Debug)]
pub struct OperationWithVariables {
    pub operation: Vec<Token>,
    pub token: Token,
}

#[derive(Debug)]
pub struct ScopeDirective {
    pub value: String,
    pub token: Token,
}

#[derive(Debug)]
pub enum Node {
    StringLiteral(StringLiteral),
    NumericLiteral(NumericLiteral),
    BooleanLiteral(BooleanLiteral),
    OperationWithVariables(OperationWithVariables),
    Literal(Literal),
    Other(Other),
    Metadata(Metadata),
    ScopeDirective(ScopeDirective),
    Style(Style),
}

pub fn except(tokens: &mut Vec<Token>, token_type: TokenType) -> Token {
    if tokens.is_empty() {
        error!("SPLX-001: Unexpected end of file");
    }
    match &tokens[0].token_type {
        token_type => eat(tokens),
        _ => panic!("SPLX-002: Unexpected token found at parsing")
    }
}

pub fn eat(tokens: &mut Vec<Token>) -> Token {
    if tokens.is_empty() {
        error!("SPLX-003: Unexpected end of file");
    }
    tokens.remove(0)
}

pub fn clone_tokens(tokens: &Vec<Token>) -> Vec<Token> {
    let mut cloned_tokens: Vec<Token> = Vec::new();
    for token in tokens {
        cloned_tokens.push(token.clone());
    }
    cloned_tokens
}

pub fn ignore_whitespace(tokens: Vec<Token>) -> bool {
    return tokens[0].token_type == TokenType::NewLine;
}

impl Clone for Node {
    fn clone(&self) -> Node {
        match self {
            Node::StringLiteral(string_literal) => {
                Node::StringLiteral(StringLiteral {
                    value: string_literal.value.clone(),
                    token: string_literal.token.clone(),
                })
            }
            Node::NumericLiteral(numeric_literal) => {
                Node::NumericLiteral(NumericLiteral {
                    value: numeric_literal.value,
                    token: numeric_literal.token.clone(),
                })
            }
            Node::Style(style) => {
                Node::Style(Style {
                    value: style.value.clone(),
                    token: style.token.clone(),
                })
            }
            Node::BooleanLiteral(boolean_literal) => {
                Node::BooleanLiteral(BooleanLiteral {
                    value: boolean_literal.value,
                    token: boolean_literal.token.clone(),
                })
            }
            Node::OperationWithVariables(operation_with_variables) => {
                Node::OperationWithVariables(OperationWithVariables {
                    operation: clone_tokens(&operation_with_variables.operation),
                    token: operation_with_variables.token.clone(),
                })
            }
            Node::Literal(literal) => {
                Node::Literal(Literal {
                    value: literal.value.clone(),
                    token: literal.token.clone(),
                })
            }
            Node::Other(other) => {
                Node::Other(Other {
                    value: other.value.clone(),
                    token: other.token.clone(),
                })
            }
            Node::Metadata(metadata) => {
                Node::Metadata(Metadata {
                    value: metadata.value.clone(),
                    token: metadata.token.clone(),
                })
            }
            Node::ScopeDirective(scope_directive) => {
                Node::ScopeDirective(ScopeDirective {
                    value: scope_directive.value.clone(),
                    token: scope_directive.token.clone(),
                })
            }
        }
    }
}

impl Literal {
    pub fn from_generic(node: Node) -> Literal {
        match node {
            Node::Literal(literal) => literal,
            _ => panic!("SPK-003: Unexpected node found at parsing")
        }
    }
}

impl Node {
    pub fn get_value(&self) -> String {
        match self {
            Node::StringLiteral(string_literal) => string_literal.value.clone(),
            Node::NumericLiteral(numeric_literal) => numeric_literal.value.to_string(),
            Node::BooleanLiteral(boolean_literal) => boolean_literal.value.to_string(),
            Node::OperationWithVariables(operation_with_variables) => {
                let mut value = String::new();
                for token in &operation_with_variables.operation {
                    value += &token.value;
                }
                value
            }
            Node::Literal(literal) => literal.value.clone(),
            Node::Other(other) => other.value.clone(),
            Node::Metadata(metadata) => {
                let mut value = String::new();
                for token in &metadata.value {
                    value += &token.value;
                }
                value
            }
            Node::ScopeDirective(scope_directive) => scope_directive.value.clone(),
            Node::Style(style) => style.value.clone(),
        }
    }
}
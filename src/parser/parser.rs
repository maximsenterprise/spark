// parser.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use crate::{
    error, 
    interpreter::interpreter_utils::Scope, 
    lexer::tokens::{Token, TokenType}, 
    parser::nodes::{BooleanLiteral, Literal, Node, NumericLiteral, Other, StringLiteral}
};

use super::{
    nodes::{eat, ignore_whitespace, Metadata, ScopeDirective, Style}, 
    operations::{self, make_operation}
};

pub fn parse(tokens: &mut Vec<Token>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut current_scope = Scope::Top;

    'parser: loop {
        if tokens.is_empty() { break; }
        let current_token = eat(tokens);

        match current_token.token_type {
            TokenType::OpenBracket => {
                if current_scope == Scope::Content {
                    let mut style = String::new();
                    while let Some(token) = tokens.first() {
                        if token.token_type == TokenType::ClosingBracket || token.token_type == TokenType::EndOfTheFile {
                            break;
                        }
                        style += &eat(tokens).value;
                    }

                    let style_node = Style {
                        value: style,
                        token: current_token,
                    };

                    nodes.push(Node::Style(style_node));
                } else {
                    let mut metadata: Vec<Token> = Vec::new();
                    let mut temp_tokens = tokens.clone();
                    if temp_tokens.len() >= 4 {
                        if let Some(first_token) = temp_tokens.get(0) {
                            if first_token.token_type == TokenType::Identifier || ignore_whitespace(temp_tokens.clone()) {
                                metadata.push(eat(tokens));
                                if let Some(second_token) = temp_tokens.get(1) {
                                    if second_token.token_type == TokenType::Colons || ignore_whitespace(temp_tokens.clone()) {
                                        metadata.push(eat(tokens));
                                        if let Some(third_token) = temp_tokens.get(2) {
                                            if third_token.token_type == TokenType::String || ignore_whitespace(temp_tokens.clone()) {
                                                metadata.push(eat(tokens));
                                                if let Some(fourth_token) = temp_tokens.get(3) {
                                                    if fourth_token.token_type == TokenType::ClosingBracket || ignore_whitespace(temp_tokens.clone()) {
                                                        eat(tokens);
                                                        let metadata_node: Metadata = Metadata {
                                                            value: metadata,
                                                            token: current_token,
                                                        };
                                                        nodes.push(Node::Metadata(metadata_node));
                                                    } else {
                                                        error!("SPK-006: Invalid metadata found at parsing: {:?}. At line: {}", tokens[0].token_type, tokens[0].line);
                                                    }
                                                }
                                            } else {
                                                error!("SPK-007: Invalid metadata found at parsing: {:?}. At line: {}", tokens[0].token_type, tokens[0].line);
                                            }
                                        }
                                    } else {
                                        error!("SPK-008: Invalid metadata found at parsing: {:?}. At line: {}", tokens[0].token_type, tokens[0].line);
                                    }
                                }
                            } else {
                                //TODO: Implement arrays
                            }
                        }
                    } else {
                        error!("SPK-009: Invalid metadata found at parsing: {:?}. At line: {}", tokens[0].token_type, tokens[0].line);
                    }
                }
            }
            TokenType::NewLine => {
                continue;
            }
            TokenType::String => {
                let string_literal = StringLiteral {
                    value: current_token.value.clone(),
                    token: current_token,
                };
                nodes.push(Node::StringLiteral(string_literal));
            }
            TokenType::EndOfTheFile => {
                break;
            }
            TokenType::Identifier => {
                let literal = Literal {
                    value: current_token.value.clone(),
                    token: current_token,
                };
                nodes.push(Node::Literal(literal));
            }
            TokenType::Number => {
                tokens.insert(0, current_token);
                nodes.push(make_operation(tokens));
            }
            TokenType::ScopeDirective => {
                if current_token.value == "content" {
                    current_scope = Scope::Content;
                }
                let scope = ScopeDirective {
                    value: current_token.value.clone(),
                    token: current_token,
                };
                nodes.push(Node::ScopeDirective(scope));
            } 
            _ => {
                let other_value = current_token.clone().value;
                let other = Other {
                    value: other_value,
                    token: current_token,
                };
                nodes.push(Node::Other(other));
            }
        }
    }

    nodes
}

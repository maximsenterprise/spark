// iframe.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use crate::{error, formatter::formatter::{format_argument, format_style}, interpreter::interpreter_utils::{eat, except, except_token_type, parse_string}, lexer::tokens::TokenType, parser::nodes::Node, utils::error};

pub fn iframe(nodes: &mut Vec<Node>) -> String {
    let mut content = String::new();
    let mut style = String::new();
    let mut alias = String::new();
    let mut arguments = String::new();

    except_token_type(nodes, TokenType::OpenParentheses);
    content = parse_string(nodes);

    if nodes.len() == 0 {
        error!("SPK-013: Unexpected end of file");
    }
    match nodes[0].clone() {
        Node::Other(other) => {
            if other.token.token_type == TokenType::Comma {
                eat(nodes);
                let mut value = String::new();
                loop {
                    if nodes.is_empty() {
                        break;
                    }
                    match nodes[0].clone() {
                        Node::Literal(literal) => {
                            let aspect_name = literal.value;
                            if aspect_name == "alias" {
                                eat(nodes);
                                except_token_type(nodes, TokenType::Colons);
                                let string_val = parse_string(nodes);
                                alias = string_val;
                                continue;
                            }
                            eat(nodes);
                            except_token_type(nodes, TokenType::Colons);
                            let string_val = parse_string(nodes); 

                            value += format_argument(aspect_name, string_val).as_str();
                        }
                        Node::Other(other) => {
                            if other.token.token_type == TokenType::Comma {
                                eat(nodes);
                                continue;
                            }
                            else {
                                break;
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                }
                arguments = value;

            }
        }
        _ => {}
    }

    except_token_type(nodes, TokenType::ClosingParentheses);
    if !nodes.is_empty() {
        match nodes[0].clone() {
            Node::Style(style_node) => {
                eat(nodes);
                let full_style = format_style(style_node.value);
                style = full_style;
            }
            _ => {}
        }
    }

    if style == "" {
        if alias == "" {
            format!("<iframe{} src=\"{}\"></a>", arguments, content)
        }
        else {
            format!("<iframe{} src=\"{}\">{}</a>", arguments, content, alias)
        }
    }
    else {
        if alias == "" {
            format!("<iframe{} style=\"{}\" src=\"{}\"></a>", arguments, style, content)
        }
        else {
            format!("<iframe{} src=\"{}\" style=\"{}\">{}</a>", arguments, content, style, alias)
        }
    }
}
// title.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use crate::{error, formatter::formatter::{format_argument, format_style}, interpreter::interpreter_utils::{eat, except, except_token_type, parse_string}, lexer::tokens::TokenType, parser::nodes::Node, utils::error};

pub fn title(nodes: &mut Vec<Node>) -> String {
    let mut title = String::new();
    let mut component_type = String::new();
    let mut style = String::new();
    let mut arguments = String::new();

    except_token_type(nodes, TokenType::OpenParentheses);
    let value = parse_string(nodes);

    if nodes.len() == 0 {
        error!("SPK-012: Unexpected end of file");
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
            Node::Other(other) => {
                if other.token.token_type == TokenType::LessThan {
                    eat(nodes);
                    component_type = String::from("h1");
                    match eat(nodes) {
                        Node::Literal(literal) => {
                            component_type = literal.value;
                        }
                        _ => {
                            error!("Unexpected node found at parsing");
                        }
                    }
                    except_token_type(nodes, TokenType::MajorThan);
                    if nodes.len() > 0 {
                        match nodes[0].clone() {
                            Node::Style(style_node) => {
                                style = style_node.value;
                            }
                            _ => {}
                        }
                    }
                    else {
        
                    }
                }
                else {
    
                }
            }
            Node::Style(style_node) => {
                eat(nodes);
                let full_style = format_style(style_node.value);
                style = full_style;
            }
            _ => {
                
            }
        }
    }
    
    if component_type.starts_with("h") {
        if style == "" {
            let tag = format!("<{}{}>{}</{}>", component_type, arguments, value, component_type);
            title = tag;
        } else {
            let tag = format!("<{}style=\"{}\"{}>{}</{}>", component_type, style, arguments, value, component_type);
            title = tag;
        }
    } else {
        if style == "" {
            let tag = format!("<h1{}>{}</h1>", arguments, value);
            title = tag;
        } else {
            let tag = format!("<h1 style=\"{}\"{}>{}</h1>", style, arguments, value);
            title = tag;
        }
    }

    return title;
}
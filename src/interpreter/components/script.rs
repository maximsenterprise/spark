// script.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use crate::{error, formatter::formatter::format_argument, interpreter::interpreter_utils::{eat, except_token_type, parse_string}, lexer::tokens::{Token, TokenType}, parser::nodes::Node, };

pub fn script(nodes: &mut Vec<Node>) -> String {
    let mut content = String::new();
    let mut arguments = String::new();
    let mut is_from_root = false; 
    match nodes[0].clone() {
        Node::Other(other) => {
            if other.token.token_type == TokenType::Script {
                eat(nodes);
                is_from_root = true;
                content = other.value;
                if nodes.is_empty() {
                    return format!("<script>{}</script>", content);
                }
                match nodes[0].clone() {
                    Node::Other(other) => {
                        if other.token.token_type == TokenType::OpenParentheses {
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
                        else {}
                    }
                    _ => {}
                }
            }   
            else if other.token.token_type == TokenType::OpenParentheses {
                is_from_root = false;
                eat(nodes);
                let script_path = {
                    match nodes[0].clone() {
                        Node::StringLiteral(string_literal) => {
                            eat(nodes);
                            content = string_literal.value;
                            if nodes.is_empty() {
                                return format!("<script src=\"{}\"></script>", content);
                            }
                            match nodes[0].clone() {
                                Node::Other(other) => {
                                    if other.token.token_type == TokenType::OpenParentheses {
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
                                    else {}
                                }
                                _ => {}
                            }
                        }
                        _ => {
                            error!("SPK-014: Unexpected node found at parsing")
                        }
                    }
                };


            }
        }
        _ => {
            error!("SPK-015: Unexpected node found at parsing");
        }
    }

    if is_from_root {
        if arguments == "" {
            format!("<script>{}</script>", content)
        }
        else {
            format!("<script {}>{}</script>", arguments, content)
        }
    }
    else {
        if arguments == "" {
            format!("<script src=\"{}\"></script>", content)
        }
        else {
            format!("<script src=\"{}\" {}></script>", content, arguments)
        }
    }
}
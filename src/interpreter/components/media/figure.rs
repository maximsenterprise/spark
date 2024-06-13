// figure.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use crate::{formatter::formatter::format_argument, interpreter::{interpreter::{content, run, FINALHTML, INNER_HTML}, interpreter_utils::{eat, except_token_type, parse_string}}, lexer::{lexer::tokenize, tokens::TokenType}, parser::{nodes::Node, parser::parse}};

pub fn figure(nodes: &mut Vec<Node>, add_to_dom: bool) -> String {
    let mut arguments = String::new();
    let mut style = String::new();
    let script = except_token_type(nodes, TokenType::Script);

    if add_to_dom == true {
        let mut tokens = tokenize(script.value);
        let mut script_nodes = parse(&mut tokens);
        run(&mut script_nodes, false);
        if nodes.len() == 0 {
            return "<figure>\n".to_string() + INNER_HTML.lock().unwrap().clone().as_str() + "\t</figure>";
        }
        else {
            loop {
                if nodes.len() == 0 {
                    break;
                }
                match nodes[0].clone() {
                    Node::Style(style_node) => {
                        eat(nodes);
                        eat(nodes);
                        style = style_node.value;
                    }
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
                            except_token_type(nodes, TokenType::ClosingParentheses);
                            arguments = value;
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
            
        }
    }
    else {
        let mut background = String::new();
        let mut tokens = tokenize(script.value);
        let mut script_nodes = parse(&mut tokens);
        loop {
            if script_nodes.len() == 0 {
                break;
            }
            else {
                background.push_str(content(&mut script_nodes, add_to_dom).as_str());
                background.push_str("\n");
            }
        }
        return "<figure>\n".to_string() + &background + "\t</figure>";
    }

    
    let content = INNER_HTML.lock().unwrap().to_string(); INNER_HTML.lock().unwrap().clear();
    if style == "" {
        return format!("<figure{}>{}</figure>", &arguments, &content);
    }
    else {
        return format!("<figure{} style=\"{}\">{}</figure>", &arguments, &style, &content);
    }
}
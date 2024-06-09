// interpreter_utils.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use crate::{error, formatter::formatter::format_html, lexer::{lexer::tokenize, tokens::TokenType}, parser::{nodes::{Literal, Node, Other}, parser::parse}, program::{get_variable, PROGRAM}};

use super::interpreter::{process_inline_elements, FINALHTML, INNER_HTML};

pub fn eat(nodes: &mut Vec<Node>) -> Node {
    nodes.remove(0)
}

pub fn except(nodes: &mut Vec<Node>, node_type: Node) -> Node {
    if nodes.is_empty() {
        panic!("SPK-011: Unexpected end of file");
    }
    match &nodes[0] {
        node_type => eat(nodes),
        _ => panic!("SPK-017: Unexpected node found at parsing")
    }
}

pub fn add_html(html: &str, add_to_dom: bool) {
    if add_to_dom == true {
        let html_formatted = format_html(html.to_string());
        let mut final_html = FINALHTML.lock().unwrap();
        final_html.push_str(&html_formatted.as_str());
    }
    else {
        let html_formatted = format_html(html.to_string());
        let mut inner_html = INNER_HTML.lock().unwrap();
        inner_html.push_str(&html_formatted.as_str());
    }
}

pub fn add_root_html(html: &str) {
    let mut final_html = FINALHTML.lock().unwrap();
    let formatted = html.to_string() + "\n";
    final_html.push_str(&formatted);
}

pub fn except_token_type(nodes: &mut Vec<Node>, token_type: TokenType) -> Other {
    if nodes.is_empty() {
        panic!("SPK-010: Unexpected end of file");
    }
    match &nodes[0] {
        Node::Other(other) => {
            if other.token.token_type == token_type {
                match eat(nodes) {
                    Node::Other(other) => other,
                    _ => error!("SPK-014: Unexpected node found at parsing: {:?}", nodes[0])
                }
            } else {
                error!("SPK-015: Unexpected node found at parsing: {:?}", nodes[0])
            }
        }
        _ => error!("SPK-016: Unexpected node found at parsing: {:?}", nodes[0])
    }
}

#[derive(Debug, PartialEq)]
pub enum Scope {
    Top,
    Content,
    Data,
}

pub fn parse_string(nodes: &mut Vec<Node>) -> String {

    let mut full_value = String::new();

    'string: loop {
        if nodes.len() == 0 {
            break;
        }
        
        match nodes[0].clone() {
            Node::StringLiteral(string) => {
                eat(nodes);
                let mut i = 0;
                let mut value = string.value.clone();
                loop {
                    if i == string.value.len() {
                        break;
                    }

                    let c = string.value.chars().nth(i).unwrap();
                    if c == '$' {
                        if i + 1 < string.value.len() {
                            if string.value.chars().nth(i + 1).unwrap().is_alphabetic() {
                                i += 2;
                                let mut var_name = String::new(); 
                                while string.value.chars().nth(i).unwrap().is_alphabetic() {
                                    if i == string.value.len() {
                                        break;
                                    }
                                    var_name.push(string.value.chars().nth(i).unwrap());
                                    i += 1;
                                }

                                value.push_str(get_variable(var_name).as_str());
                            }
                            else if string.value.chars().nth(i + 1).unwrap() == '(' {
                                i += 1;
                                let mut var_name = String::new();
                                while string.value.chars().nth(i).unwrap() != ')' {
                                    if i == string.value.len() {
                                        break;
                                    }
                                    var_name.push(string.value.chars().nth(i).unwrap());
                                    i += 1;
                                }

                                let mut inline_nodes = parse(&mut tokenize(var_name)); 
                                let first = inline_nodes[0].clone();
                                let mut inner_html = process_inline_elements(&mut inline_nodes, Literal::from_generic(first));
                                value.push_str(inner_html.as_str());
                            }
                        }
                    }
                    else {
                        value.push(c);
                    }

                    i += 1;
                    
                }
            }
            Node::Other(other) => {
                if other.token.token_type == TokenType::Plus {
                    eat(nodes);
                    continue;
                }
                else {
                    break 'string;
                }
            }
            _ => {
                break 'string;
            }
        }
    }

    return full_value;
}
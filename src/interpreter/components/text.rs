// text.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use crate::{error, interpreter::interpreter_utils::{eat, except_token_type}, lexer::tokens::TokenType, parser::nodes::Node};

pub fn text(nodes: &mut Vec<Node>) -> String {
    let mut content = String::new();
    except_token_type(nodes, TokenType::OpenParentheses);
    match nodes[0].clone() {
        Node::StringLiteral(literal) => {
            eat(nodes);
            content = literal.value;
        }
        _ => {
            error!("SPK-014: Expected a string literal");
        }
    }
    except_token_type(nodes, TokenType::ClosingParentheses);
   content
}
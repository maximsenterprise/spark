// operations.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use std::vec::Vec;
use crate::error;
use crate::lexer::tokens::{Token, TokenType};
use crate::parser::nodes::{Node, StringLiteral, NumericLiteral, BooleanLiteral, Literal, Other, eat};

use super::nodes::OperationWithVariables;

pub fn make_operation(tokens: &mut Vec<Token>) -> Node {
    let mut operation = Vec::new();
    let mut i = 0;
    let mut token = tokens[i].clone();
    let mut hasVariables: bool = false;
    let mut opened_parenthesis = 0;
    loop {
        token = tokens[i].clone();
        if token.token_type == TokenType::EndOfTheFile {
            break;
        }
        else if token.token_type == TokenType::NewLine {
            i += 1;
            continue;
        }
        else if token.token_type == TokenType::OpenParentheses {
            opened_parenthesis += 1;
            operation.push(eat(tokens));
        }
        else if token.token_type == TokenType::ClosingParentheses {
            opened_parenthesis -= 1;
            operation.push(eat(tokens));
        }
        else if token.token_type == TokenType::Identifier {
            hasVariables = true;
            operation.push(eat(tokens));
        }
        else if token.token_type == TokenType::Minus {
            operation.push(eat(tokens));
        }
        else if token.token_type == TokenType::Number {
            operation.push(eat(tokens));
        }
        else if token.token_type == TokenType::DecimalNumber {
            operation.push(eat(tokens));
        }
        else if token.token_type == TokenType::Plus {
            operation.push(eat(tokens));
        }
        else if token.token_type == TokenType::Multiply {
            operation.push(eat(tokens));
        }
        else if token.token_type == TokenType::Divide {
            operation.push(eat(tokens));
        }
        else if token.token_type == TokenType::Modulus {
            operation.push(eat(tokens));
        }
        else if hasVariables {
            operation.push(eat(tokens));
        }
        else {
            break;
        }
    }

    if opened_parenthesis != 0 {
        error!("Parenthesis not closed");
    }

    if hasVariables {
        Node::OperationWithVariables(OperationWithVariables{
            operation: operation.clone(),
            token: operation[0].clone(),
        })
    }
    else {
        Node::NumericLiteral(solve(&mut operation))
    }
}

fn solve(operation: &mut Vec<Token>) -> NumericLiteral {
    let mut new_operation = solve_parenthesis(operation);
    new_operation = solve_top_level_arithemtic(&mut new_operation);
    solve_low_level_arithemtic(&mut new_operation)
}

fn solve_parenthesis(operation: &mut Vec<Token>) -> Vec<Token> {
    let mut i = 0;
    let mut opened_parenthesis = 0;
    let mut closed_parenthesis = 0;
    let mut new_operation = Vec::new();
    loop {
        if i == operation.len() {
            break;
        }
        let token = operation[i].clone();
        if token.token_type == TokenType::OpenParentheses {
            let mut inner_operation = Vec::new();
            opened_parenthesis += 1;
            let mut j = i + 1;
            loop {
                let inner_token = operation[j].clone();
                if inner_token.token_type == TokenType::ClosingParentheses {
                    closed_parenthesis += 1;
                }
                if opened_parenthesis == closed_parenthesis {
                    break;
                }
                inner_operation.push(inner_token);
                j += 1;
            }
            i = j;
            let inner_result = solve(&mut inner_operation);
            new_operation.push(Token::new(TokenType::Number, inner_result.value.to_string(), token.line));
            continue;
        }
        else if token.token_type == TokenType::ClosingParentheses {
            closed_parenthesis += 1;
        }
        new_operation.push(token);
        i += 1;
    }
    new_operation
}

fn solve_top_level_arithemtic(operation: &mut Vec<Token>) -> Vec<Token> {
    let mut i = 0;
    let mut new_operation = Vec::new();
    loop {
        if i == operation.len() {
            break;
        }
        let current_token = operation[i].clone();
        if current_token.token_type == TokenType::Multiply {
            let left: Token = new_operation.pop().unwrap();
            let right: Token = operation[i + 1].clone();
            let result = left.value.parse::<f64>().unwrap() * right.value.parse::<f64>().unwrap();
            new_operation.push(Token::new(TokenType::Number, result.to_string(), left.line));
            i += 1;
        }
        else if current_token.token_type == TokenType::Divide {
            let left = new_operation.pop().unwrap();
            let right = operation[i + 1].clone();
            let result = left.value.parse::<f64>().unwrap() / right.value.parse::<f64>().unwrap();
            new_operation.push(Token::new(TokenType::Number, result.to_string(), left.line));
            i += 1;
        }
        else if current_token.token_type == TokenType::Modulus {
            let left = new_operation.pop().unwrap();
            let right = operation[i + 1].clone();
            let result = left.value.parse::<f64>().unwrap() % right.value.parse::<f64>().unwrap();
            new_operation.push(Token::new(TokenType::Number, result.to_string(), left.line));
            i += 1;
        }
        else {
            new_operation.push(current_token);
        }
        i += 1;
    }

    new_operation
}

fn solve_low_level_arithemtic(operation: &mut Vec<Token>) -> NumericLiteral {
    let mut i = 0;
    let mut result: f64 = 0.0;
    for (token, index) in operation.clone().iter().zip(0..operation.len()) {
        if token.token_type == TokenType::ClosingParentheses {
            operation.remove(index);
        }
    }
    if operation.len() == 1 {
        return NumericLiteral {
            value: operation[0].value.parse::<f64>().unwrap(),
            token: operation[0].clone(),
        }
    }
    loop {
        if i == operation.len() {
            break;
        }
        let current_token = operation[i].clone();
        if current_token.token_type == TokenType::Plus {
            let left = operation[i - 1].clone();
            let right = operation[i + 1].clone();
            result = left.value.parse::<f64>().unwrap() + right.value.parse::<f64>().unwrap();
            i += 1;
        }
        else if current_token.token_type == TokenType::Minus {
            let left = result;
            let right = operation[i + 1].clone();
            result = left - right.value.parse::<f64>().unwrap();
            i += 1;
        }
        i += 1;
    }

    NumericLiteral {
        value: result,
        token: operation[0].clone(),
    }
}
// tokens.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use std::{collections::HashMap, hash::Hash};
use lazy_static::lazy_static;

#[derive(Debug)]
#[derive(Clone, PartialEq)]
pub enum TokenType {
    //Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,

    //Punctuation
    EqualSign,
    OpenParentheses,
    ClosingParentheses,
    Colons,
    OpenBracket,
    ClosingBracket,
    MajorThan,
    LessThan,
    Semicolon,
    Comma,
    Dot,
    ExclamationMark,

    //Keywords
    Component,
    Define,
    Use,

    //Other
    NewLine,
    EndOfTheFile,
    Number,
    DecimalNumber,
    String,
    Identifier,
    Directive,
    ScopeDirective,
    Script,
    InLineComments,
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("component", TokenType::Component);
        map.insert("define", TokenType::Define);
        map.insert("use", TokenType::Use);
        map
    };
    
    pub static ref OPERATORS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("+", TokenType::Plus);
        map.insert("-", TokenType::Minus);
        map.insert("*", TokenType::Multiply);
        map.insert("/", TokenType::Divide);
        map.insert("%", TokenType::Modulus);
        map
    };
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, line: i32) -> Token {
        Token {
            token_type,
            value,
            line
        }
    }

    pub fn print(&self) {
        println!("Token: {:?}, Value: {}", self.token_type, self.value);
    }

    pub fn clone(&self) -> Token {
        Token {
            token_type: self.token_type.clone(),
            value: self.value.clone(),
            line: self.line
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Token {
        Token {
            token_type: self.token_type.clone(),
            value: self.value.clone(),
            line: self.line
        }
    }
}
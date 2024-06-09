use crate::lexer::tokens::{Token, TokenType, KEYWORDS};
use crate::error;

// lexer.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

pub fn tokenize(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0;
    let mut line = 1;
    
    loop {
        if i >= code.len() {
            break;
        }
        
        let current_char = code.chars().nth(i).unwrap();
        
        if current_char == '\n' {
            tokens.push(Token::new(TokenType::NewLine, current_char.to_string(), line));
            line += 1;
        }
        else if current_char.is_whitespace() {
            
        }
        else if current_char == '+' {
            tokens.push(Token::new(TokenType::Plus, current_char.to_string(), line));
        }
        else if current_char == '-' {
            tokens.push(Token::new(TokenType::Minus, current_char.to_string(), line));
        }
        else if current_char == '*' {
            tokens.push(Token::new(TokenType::Multiply, current_char.to_string(), line));
        }
        else if current_char == '!' {
            tokens.push(Token::new(TokenType::ExclamationMark, current_char.to_string(), line));
        }
        else if current_char == '/' {
            if code.len() > i + 1 && code.chars().nth(i + 1).unwrap() == '/' {
                let mut j = i + 2;
                let mut comment = String::new();
                
                loop {
                    let current_char = match code.chars().nth(j) {
                        Some(c) => c,
                        None => {
                            break;
                        }
                    };
                    
                    if current_char == '\n' {
                        break;
                    }
                    
                    comment.push(current_char);
                    
                    j += 1;
                }
                
                i = j;

                tokens.push(Token::new(TokenType::InLineComments, comment, line));

            }
            else {
                tokens.push(Token::new(TokenType::Divide, current_char.to_string(), line));
            }
            
        }
        else if current_char == '=' {
            tokens.push(Token::new(TokenType::EqualSign, current_char.to_string(), line));
        }
        else if current_char == '(' {
            tokens.push(Token::new(TokenType::OpenParentheses, current_char.to_string(), line));
        }
        else if current_char == ')' {
            tokens.push(Token::new(TokenType::ClosingParentheses, current_char.to_string(), line));
        }
        else if current_char == '[' {
            tokens.push(Token::new(TokenType::OpenBracket, current_char.to_string(), line));
        }
        else if current_char == ',' {
            tokens.push(Token::new(TokenType::Comma, current_char.to_string(), line));
        }
        else if current_char == ']' {
            tokens.push(Token::new(TokenType::ClosingBracket, current_char.to_string(), line));
        }
        else if current_char == ':' {
            tokens.push(Token::new(TokenType::Colons, current_char.to_string(), line));
        }
        else if current_char == ';' {
            tokens.push(Token::new(TokenType::Semicolon, current_char.to_string(), line));
        }
        else if current_char == '{' {
            let mut j = i + 1;
            let mut directive = String::new();
            let mut opened_brackets = 1;
            
            loop {
                let current_char = match code.chars().nth(j) {
                    Some(c) => c,
                    None => {
                        break;
                    }
                };
                
                if current_char == '}' {
                    opened_brackets -= 1;
                    if opened_brackets == 0 {
                        break;
                    }
                }
                else if current_char == '{' {
                    opened_brackets += 1;
                }
                
                directive.push(current_char);
                j += 1;
            }

            i = j;

            tokens.push(Token::new(TokenType::Script, directive, line))
        }
        else if current_char == '>' {
            tokens.push(Token::new(TokenType::MajorThan, current_char.to_string(), line));
        }
        else if current_char == '<' {
            tokens.push(Token::new(TokenType::LessThan, current_char.to_string(), line));
        }
        else if current_char == '.' {
            tokens.push(Token::new(TokenType::Dot, current_char.to_string(), line));
        }
        else if current_char == '"' {
            let mut string = String::new();
            let mut j = i + 1;
            
            loop {
                let current_char = match code.chars().nth(j) {
                    Some(c) => c,
                    None => {
                        break;
                    }
                };
                
                if current_char == '"' {
                    break;
                }
                
                string.push(current_char);
                j += 1;
            }
            
            i = j;
            tokens.push(Token::new(TokenType::String, string, line));
        }
        else if current_char == '@' {
            let mut scope_dir = String::new();
            let mut j = i + 1;
            
            loop {
                let current_char = match code.chars().nth(j) {
                    Some(c) => c,
                    None => {
                        break;
                    }
                };
                
                if current_char == '\n' {
                    break;
                }
                
                scope_dir.push(current_char);
                j += 1;
            }
            
            i = j;
            tokens.push(Token::new(TokenType::ScopeDirective, scope_dir, line));
        }
        else if current_char.is_digit(10) {
            let mut number = String::new();
            let mut is_decimal = false;
            let mut j = i;
            
            loop {
                let current_char = match code.chars().nth(j) {
                    Some(c) => c,
                    None => {
                        break;
                    }
                };
                
                if current_char.is_digit(10) {
                    number.push(current_char);
                }
                else if current_char == '.' && !is_decimal {
                    number.push(current_char);
                    is_decimal = true;
                }
                else {
                    break;
                }
                
                j += 1;
            }
            
            i = j - 1;
            
            if is_decimal {
                tokens.push(Token::new(TokenType::DecimalNumber, number, line));
            }
            else {
                tokens.push(Token::new(TokenType::Number, number, line));
            }
        }
        else if current_char.is_alphabetic() || current_char == '_' {
            let mut identifier = String::new();
            let mut j = i;
            
            loop {
                let current_char = match code.chars().nth(j) {
                    Some(c) => c,
                    None => {
                        break;
                    }
                };
                
                if current_char.is_alphanumeric() || current_char == '_' {
                    identifier.push(current_char);
                }
                else {
                    break;
                }
                
                j += 1;
            }
            
            i = j - 1;
            
            if KEYWORDS.contains_key(&identifier.as_str()) {
                tokens.push(Token::new(KEYWORDS.get(&identifier.as_str()).unwrap().clone(), identifier, line));
            }
            else {
                tokens.push(Token::new(TokenType::Identifier, identifier, line));
            }
        }
        else {
            error!("Invalid character found at parsing: {}. At line: {}", current_char, line);
        }
        
        i += 1;
    }
    
    tokens
}

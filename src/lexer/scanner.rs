use super::token::{lookup_keyword, Token};
use crate::errors::XtaError;

pub struct Scanner<'a> {
    input: &'a str,  // input is now a string slice
    position: usize, // the position that displays the current char
    offset: usize,   // a position of which the next char would be
    line: usize,
    curr: char,
}

impl<'a> Scanner<'a> {
    // returns a new instance, C'tor
    pub fn new(input: &'a str) -> Self {
        let mut scanner = Self {
            input,
            position: 0,
            line: 1,
            offset: 0,
            curr: '\0',
        };
        scanner.advance();
        scanner
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    // returns the next token
    fn next(&mut self) -> Option<Token> {
        let mut token: Token = Token::Illegal(String::new());
        self.ignore_whitespace();

        match self.curr {
            '\0' => {
                token = Token::EOF;
            }
            '+' => {
                token = Token::Plus;
                if self.peek() == '+' {
                    self.advance();
                    token = Token::Inc;
                }
            }
            '-' => {
                token = Token::Min;
                if self.peek() == '>' {
                    self.advance();
                    token = Token::ReturnTypeArrow;
                } else if self.peek() == '-' {
                    self.advance();
                    token = Token::Dec;
                }
            }
            '*' => {
                token = Token::Mul;
            }
            '/' => {
                token = Token::Div;
            }
            '(' => {
                token = Token::LeftParen;
            }
            ')' => {
                token = Token::RightParen;
            }
            ';' => {
                token = Token::Semicolon;
            }
            '{' => {
                token = Token::LeftBrace;
            }
            '}' => {
                token = Token::RightBrace;
            }
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    token = Token::Equals;
                } else {
                    token = Token::Assign;
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    token = Token::NotEquals;
                } else {
                    token = Token::Not;
                }
            }
            '~' => {
                token = Token::BNot;
            }
            '^' => {
                token = Token::Xor;
            }
            '|' => {
                if self.peek() == '|' {
                    self.advance();
                    token = Token::Or;
                } else {
                    token = Token::BOr;
                }
            }
            '&' => {
                if self.peek() == '&' {
                    self.advance();
                    token = Token::And;
                } else {
                    token = Token::BAnd;
                }
            }
            '>' => {
                if self.peek() == '>' {
                    self.advance();
                    token = Token::RightSh;
                } else if self.peek() == '=' {
                    self.advance();
                    token = Token::GreaterOrEqu;
                } else {
                    token = Token::Greater;
                }
            }
            '<' => {
                if self.peek() == '<' {
                    self.advance();
                    token = Token::LeftSh;
                } else if self.peek() == '=' {
                    self.advance();
                    token = Token::LowerOrEqu;
                } else {
                    token = Token::Lower;
                }
            }
            '"' => {
                self.advance();
                let string = self.get_string();
                token = string;
            }
            _ => {
                if self.curr.is_alphabetic() {
                    let id = self.get_identifier();

                    if id == "true" || id == "false" {
                        token = Token::Boolean(id == "true");
                    } else {
                        token = match lookup_keyword(id.as_str()) {
                            Some(keyword) => keyword,
                            None => Token::Identifier(id),
                        };
                    }
                    return Some(token); // avoid advancing the input, because it has already been advanced when get_identifier is called.
                } else if self.curr.is_numeric() {
                    // same as the identifier part, `get_number` advances the token, so no need for it.
                    return Some(self.get_number());
                }
            }
        }

        self.advance();
        Some(token)
    }
}

// implementation for private functions
impl<'a> Scanner<'a> {
    // eats up white-space
    fn ignore_whitespace(&mut self) {
        while self.curr.is_whitespace() {
            if self.curr == '\n' {
                self.line += 1;
            }
            self.advance();
        }
    }

    // moves to the next char
    fn advance(&mut self) {
        self.curr = self.peek();

        self.position = self.offset;
        self.offset += 1;
    }

    // peek next char
    fn peek(&self) -> char {
        if self.offset >= self.input.len() {
            '\0'
        } else {
            self.input[self.offset..].chars().next().unwrap_or('\0')
        }
    }

    fn get_string(&mut self) -> Token {
        let begin_pos = self.position;
        while self.peek() != '"' {
            self.advance();

            if self.peek() == '\0' {
                return Token::Illegal(self.input[begin_pos..self.position].to_string());
            }
        }

        self.advance();

        Token::String(self.input[begin_pos..self.position].to_string())
    }

    fn get_identifier(&mut self) -> String {
        let begin_pos = self.position;
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        self.advance();

        self.input[begin_pos..self.position].to_string()
    }

    fn get_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut is_floating = false;

        while self.curr.is_digit(10) || self.curr == '.' {
            if self.curr == '.' {
                if is_floating {
                    return Token::Illegal(num_str);
                }
                is_floating = true;
            }
            num_str.push(self.curr);
            self.advance();
        }

        if is_floating {
            match num_str.parse::<f64>() {
                Ok(value) => Token::Double(value),
                Err(_) => Token::Illegal(num_str),
            }
        } else {
            match num_str.parse::<i32>() {
                Ok(value) => Token::Integer(value),
                Err(_) => Token::Illegal(num_str),
            }
        }
    }
}

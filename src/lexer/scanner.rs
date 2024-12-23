use crate::errors::XtaError;

use super::token::Token;

pub struct Scanner {
    input: Vec<char>, // the input as a vector of chars, since string is kinda shitty
    position: usize,  // a position that displays the current char
    offset: usize,    // a position of which the next char would be
    line: usize,
    curr: char,
}

enum Number {
    Int(i32),
    Double(f64),
}

impl Scanner {
    pub fn new(input: &str) -> Self {
        let mut scanner = Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            offset: 0,
            curr: '\0',
        };

        scanner.advance();
        return scanner;
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, XtaError> {
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        let mut token = Token::Illegal;
        while token != Token::EOF {
            token = self.next();
            tokens.push(token.clone());
        }

        Ok(tokens)
    }
}

// implementation for private functions
impl Scanner {
    fn next(&mut self) -> Token {
        let mut token: Token = Token::Illegal;

        self.ignore_whitespace();

        match self.curr {
            '\0' => {
                token = Token::EOF;
            }
            '+' => {
                token = Token::Plus;
            }
            '-' => {
                token = Token::Min;
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
            '"' => {
                self.advance();
                let string = self.get_string();
                token = Token::String(string);
            }
            _ => {
                if self.curr.is_alphabetic() {
                    let id = self.get_identifier();
                    if id == "true" || id == "false" {
                        token = Token::Boolean(id == "true");
                    } else {
                        token = Token::Identifier(id);
                    }
                }
            }
        }

        self.advance();
        return token;
    }

    fn ignore_whitespace(&mut self) {
        while self.curr == ' ' || self.curr == '\n' || self.curr == '\t' {
            if self.curr == '\n' {
                self.line += 1;
            }

            self.advance();
        }
    }

    // moves to the next char
    fn advance(&mut self) {
        if self.offset >= self.input.len() {
            self.curr = '\0';
        } else {
            self.curr = self.input[self.offset];
        }

        self.position = self.offset;
        self.offset += 1;
    }

    // peek next char
    fn peek(&mut self) -> char {
        if self.offset >= self.input.len() {
            '\0'
        } else {
            self.input[self.offset]
        }
    }

    fn get_string(&mut self) -> String {
        let begin_pos = self.position;
        while self.peek() != '"' {
            self.advance();
        }

        self.advance();

        let buf = &self.input[begin_pos..self.position];
        buf.to_vec().iter().collect()
    }

    fn get_identifier(&mut self) -> String {
        let begin_pos = self.position;
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        self.advance();

        let buf = &self.input[begin_pos..self.position];
        buf.to_vec().iter().collect()
    }

    fn get_number(&mut self) -> Result<Number, XtaError> {
        let mut num_str = String::from("");
        let mut is_floating = false;

        while self.curr.is_digit(10) || self.curr == '.' {
            if self.curr == '.' {
                if is_floating {
                    return Err(XtaError::ScannerError(self.curr, self.line));
                }

                is_floating = true;
            }

            num_str.push(self.curr);
            self.advance();
        }

        // TODO: fix
        if is_floating {
            num_str
                .parse::<f64>()
                .map(Number::Double)
                .map_err(|_| XtaError::InvalidNumberFormat(num_str))
        } else {
            num_str
                .parse::<i32>()
                .map(Number::Int)
                .map_err(|_| XtaError::InvalidNumberFormat(num_str))
        }

        Ok(Number::Int(20))
    }
}

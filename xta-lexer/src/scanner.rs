use crate::token::{lookup_keyword, Loc, Token, TokenKind};

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
            line: 0,
            offset: 0,
            curr: '\0',
        };
        scanner.advance();
        scanner
    }
}


// implementation for private functions
impl<'a> Scanner<'a> {
    pub fn next_token(&mut self) -> Token {
        let mut token: TokenKind = TokenKind::Illegal;
        let mut loc = self.get_loc();
        let mut content = "";
        self.ignore_whitespace();

        match self.curr {
            '\0' => {
                token = TokenKind::EOF;
            }
            ',' => {
                token = TokenKind::Comma;
            }
            '+' => {
                token = TokenKind::Plus;
                if self.peek() == '+' {
                    self.advance();
                    token = TokenKind::Inc;
                }
            }
            '-' => {
                token = TokenKind::Min;
                if self.peek() == '>' {
                    self.advance();
                    token = TokenKind::ReturnTypeArrow;
                } else if self.peek() == '-' {
                    self.advance();
                    token = TokenKind::Dec;
                }
            }
            '*' => {
                token = TokenKind::Mul;
            }
            '/' => {
                token = TokenKind::Div;
            }
            '(' => {
                token = TokenKind::LeftParen;
            }
            ')' => {
                token = TokenKind::RightParen;
            }
            ';' => {
                token = TokenKind::Semicolon;
            }
            '{' => {
                token = TokenKind::LeftBrace;
            }
            '}' => {
                token = TokenKind::RightBrace;
            }
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    token = TokenKind::Equals;
                } else {
                    token = TokenKind::Assign;
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    token = TokenKind::NotEquals;
                } else {
                    token = TokenKind::Not;
                }
            }
            '~' => {
                token = TokenKind::BNot;
            }
            '^' => {
                token = TokenKind::Xor;
            }
            '|' => {
                if self.peek() == '|' {
                    self.advance();
                    token = TokenKind::Or;
                } else {
                    token = TokenKind::BOr;
                }
            }
            '&' => {
                if self.peek() == '&' {
                    self.advance();
                    token = TokenKind::And;
                } else {
                    token = TokenKind::BAnd;
                }
            }
            '>' => {
                if self.peek() == '>' {
                    self.advance();
                    token = TokenKind::RightSh;
                } else if self.peek() == '=' {
                    self.advance();
                    token = TokenKind::GreaterOrEqu;
                } else {
                    token = TokenKind::Greater;
                }
            }
            '<' => {
                if self.peek() == '<' {
                    self.advance();
                    token = TokenKind::LeftSh;
                } else if self.peek() == '=' {
                    self.advance();
                    token = TokenKind::LowerOrEqu;
                } else {
                    token = TokenKind::Lower;
                }
            }
            '"' => {
                return self.get_string();
            }
            _ => {
                if self.curr.is_alphabetic() {
                    let id = self.get_identifier();

                    if id == "true" || id == "false" {
                        return Token::new(TokenKind::Boolean, loc, id);
                    } else {
                        return Token::new(lookup_keyword(id.clone().as_str()), loc, id);
                    }
                } else if self.curr.is_numeric() {
                    return self.get_number();
                } else {
                    return Token::new(TokenKind::Illegal, loc, self.curr.to_string());
                }
            }
        }

        self.advance();
        Token::new(token, loc, content.to_string())
    }

    fn get_loc(&mut self) -> Loc {
        Loc {
            col: self.position as u32,
            row: self.line as u32,
        }
    }

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
        let loc = self.get_loc();

        self.advance(); // remove '"'

        while self.peek() != '"' {
            self.advance();

            if self.peek() == '\0' {
                return Token::new(TokenKind::Illegal, loc, self.input[begin_pos..self.position].to_string());
            }
        }

        self.advance();

        Token::new(TokenKind::String, loc, self.input[begin_pos..self.position].to_string())
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
        let loc = self.get_loc();
        let mut num_str = String::new();
        let mut is_floating = false;

        while self.curr.is_digit(10) || self.curr == '.' {
            if self.curr == '.' {
                if is_floating {
                    return Token::new(TokenKind::Illegal, loc, num_str);
                }
                is_floating = true;
            }
            num_str.push(self.curr);
            self.advance();
        }

        if is_floating {
            
            Token::new(TokenKind::Double, loc, num_str)
            
        } else {
            Token::new(TokenKind::Integer, loc, num_str)
        }
    }
}
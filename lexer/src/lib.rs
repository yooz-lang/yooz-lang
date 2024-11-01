use std::fmt;
use token::Token;

mod lexer_test;
mod token;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    pos: usize,
    read_pos: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        if input.len() == 0 {
            panic!("Input must not be empty!")
        }

        let first_char = input
            .clone()
            .chars()
            .nth(0)
            .expect("Failed to read first char of the given input");

        Self {
            input: input,
            pos: 0,
            read_pos: 1,
            ch: first_char,
        }
    }

    fn next_char(&mut self) {
        if self.read_pos == self.input.len() {
            self.ch = ' ';
        } else {
            self.ch = self
                .input
                .chars()
                .nth(self.read_pos)
                .expect("Failed to read nth char of the input");
        }

        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let matched_token = match self.ch {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Assign),
            '/' => Some(Token::Slash),
            '%' => Some(Token::Modulo),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '[' => Some(Token::LeftBracket),
            ']' => Some(Token::RightBracket),
            ',' => Some(Token::Comma),
            ':' => Some(Token::Colon),
            '#' => Some(Token::Hashtag),
            '"' => Some(Token::DoubleQuote),
            '_' => Some(Token::Underline),
            _ => Some(Token::Illegal),
        };

        self.next_char();
        matched_token
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self
            .next_token()
            .expect("Failed to get next token in lexer");

        println!("done: {}", self.read_pos);
        if self.read_pos > self.input.len() {
            return None;
        }

        Some(token)
    }
}

impl fmt::Display for Lexer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "pos: {}, read_pos: {}, char: {}",
            self.pos, self.read_pos, self.ch
        )
    }
}

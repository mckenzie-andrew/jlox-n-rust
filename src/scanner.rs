use crate::token::{Token, Literal};
use crate::token_type::TokenType;
use crate::error;

pub(crate) struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {
    pub(crate) fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1

        }
    }

    pub fn scan_tokens(&mut self) -> &[Token] {

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        };

        self.tokens.push(
            Token::new(
                TokenType::Eof,
                String::from(""),
                None,
                self.line
            )
        );

        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            },
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            },
            '"' => self.string(),
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            '0'..'9' => self.number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),
            _ => error(self.line as u32, "Unexpected character")
        };
    }

    fn advance(&mut self) -> char{
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, kind: TokenType) {
        self.add_token_literal(kind, None)
    }

    fn add_token_literal(&mut self, kind: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(kind, text, literal, self.line))
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source[self.current] != expected { return false; }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        self.source[self.current]
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line as u32, "Unterminated string");
            return
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1].iter().collect();
        self.add_token_literal(TokenType::String, Some(Literal::Str(value)));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() { self.advance(); }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() { self.advance(); }
        while self.peek().is_ascii_digit() { self.advance(); }
        let text: String = self.source[self.start..self.current].iter().collect();
        let value: f64 = text.parse().unwrap();
        self.add_token_literal(TokenType::Number, Some(Literal::Number(value)))
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' { self.advance(); }
        let text: String = self.source[self.start..self.current].iter().collect();
        let kind = Self::keyword(&text).unwrap_or(TokenType::Identifier);
        self.add_token(kind);
    }

    fn keyword(text: &str) -> Option<TokenType> {
        match text {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}


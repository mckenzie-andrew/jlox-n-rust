use crate::token_type::TokenType;
use std::fmt;

#[derive(Debug, Clone)]
pub(crate) enum Literal {
    Number(f64),
    Str(String)
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Str(s) => write!(f, "{}", s)
        }
    }
}


#[derive(Debug, Clone)]
pub(crate) struct Token {
    kind: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: u32,
}


impl Token {
    fn new(kind: TokenType, lexeme: String, literal: Option<Literal>, line: u32) -> Token {
        Token {
            kind,
            lexeme,
            literal,
            line
        }
    }
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let literal = match &self.literal {
            Some(lit) => lit.to_string(),
            None => "NULL".to_string(),
        };
        write!(f, "{:?} {} {}", self.kind, self.lexeme, literal)
    }
}
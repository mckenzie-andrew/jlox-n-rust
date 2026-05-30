use crate::token_type;
use crate::token::Token;

pub(crate) struct Scanner {
    source: String,
    tokens: Vec<Token>
}

impl Scanner {
    pub(crate) fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
        }
    }
}
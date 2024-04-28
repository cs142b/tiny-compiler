use crate::tokenizer::{Token, Tokenizer};

pub struct Parser {
    token: Tokenizer,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Self {
            token: Tokenizer::new(input),
        }
    }

}

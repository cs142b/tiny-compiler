use core::panic;

use crate::tokenizer::{Token, Tokenizer};

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Self {
            tokenizer: Tokenizer::new(input),
        }
    }

    fn parse_if_statement(&mut self) {
        self.match_token(Token::If);
        self.parse_relation();
        self.match_token(Token::Then);
        // TODO: implement StatsSequence
        // and so on
    }

    fn parse_assignment(&mut self) {
        self.match_token(Token::Let);
        let identifier = self.get_identifier();
        self.match_token(Token::Assignment);
        self.parse_expression();
    }

    fn parse_relation(&mut self) {
        self.parse_expression();
        let rel_op = self.get_operator();
        self.parse_expression();
    }

    /// Retrieves the next token and returns if it's a valid identifier 
    fn get_identifier(&mut self) -> Token {
        let token = self.tokenizer.next_token();

        match token {
            Token::Identifier(_) => token, 
            _ => panic!("ERROR: {:?} is not a valid identifier", token),
        }
    }

    /// Retrieves the next token and returns if it's a valid operator 
    fn get_operator(&mut self) -> Token {
        let operator_tokens = vec![Token::Equal, Token::NotEqual, Token::Greater, Token::GreaterEqual, Token::Less, Token::LessEqual];

        let token = self.tokenizer.next_token();

        if operator_tokens.contains(&token) {
            return token;
        } else {
            panic!("ERROR: {:?} is not a valid operator", token);
        }
    }

    fn parse_expression(&mut self) {
        self.parse_term();

        loop {
            let token = self.tokenizer.peek_token();
            match token {
                Token::Plus => {
                    self.tokenizer.next_token();
                }
                Token::Minus => {
                    self.tokenizer.next_token();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn parse_term(&mut self) {
        self.parse_factor();

        loop {
            let token = self.tokenizer.peek_token();
            match token {
                Token::Times => {
                    self.tokenizer.next_token();
                }
                Token::Divide => {
                    self.tokenizer.next_token();
                },
                _ => {
                    break;
                }
            }
        }
    }

    
    fn parse_factor(&mut self) {
        // NOTE: I could advance it to eliminate extra code, but does it affect readability?
        let token = self.tokenizer.peek_token();

        match token {
            Token::Identifier(name) => {
                self.tokenizer.next_token();
            },
            Token::Number(digits) => {
                self.tokenizer.next_token();
            },
            Token::OpenParen => {
                self.tokenizer.next_token();
                self.parse_expression();
                self.match_token(Token::CloseParen);
            },
            Token::FunctionCall => {
                // TODO: implement this function
            },
            _ => {
                panic!("ERROR: write ...");
            },
        }

    }


    fn match_token(&mut self, token_to_match: Token) {
        // advances regardless of token, should always match, else syntax error
        let token = self.tokenizer.next_token();
        match token {
            token_to_match => (),
            _ => panic!("ERROR: Unexpected token, expected {:?}, instead got {:?}", token_to_match, token),
        }
    }

}

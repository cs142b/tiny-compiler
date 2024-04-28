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

    fn parse_assignment(&mut self) {
        self.match_token(Token::Let);
        // get identifier
        // we would need a verify function to make sure this is an identifier
        self.match_token(Token::Assignment);
        self.parse_expression();
    }

    fn parse_relation(&mut self) {
        self.parse_expression();
        self.parse_relation_operator(); // skeleton code
        // we would need a verify function to make sure this is an relation operator
        self.parse_expression();

    }
    
    // skeleton code, make a generic function that can verify
    fn parse_relation_operator(&mut self) -> Token {
        let token = match self.token.peek_token() {
            Token::Equal | Token::NotEqual | Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual => self.token.next_token(),
            _ => panic!("ERROR: write ..."),
        };

        token
    }
    
    fn parse_expression(&mut self) {
        self.parse_term();
        loop {
            let token = self.token.peek_token();
            match token {
                Token::Plus => {
                    self.token.next_token();
                }
                Token::Minus => {
                    self.token.next_token();
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
            let token = self.token.peek_token();
            match token {
                Token::Times => {
                    self.token.next_token();
                }
                Token::Divide => {
                    self.token.next_token();
                },
                _ => {
                    break;
                }
            }
        }
    }

    
    fn parse_factor(&mut self) {
        // NOTE: I could advance it to eliminate extra code, but does it affect readability?
        let token = self.token.peek_token();

        match token {
            Token::Identifier(name) => {
                self.token.next_token();
            },
            Token::Number(digits) => {
                self.token.next_token();
            },
            Token::OpenParen => {
                self.token.next_token();
                self.parse_expression();
                self.match_token(Token::CloseParen);
            },
            Token::FunctionCall => {
                self.parse_function_call();
                todo!("do this");
            },
            _ => {
                panic!("ERROR: write ...");
            },
        }

    }


    fn match_token(&mut self, token_to_match: Token) -> Token {
        // advances regardless of token, should always match, else syntax error
        let token = self.token.next_token();
        if token != token_to_match {
            panic!("Does not match (open/close) {:?}", token_to_match);
        }
        
        // might need to preserve token?
        token
    }

}

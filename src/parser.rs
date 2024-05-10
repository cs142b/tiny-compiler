use crate::tokenizer::{Token, Tokenizer};
use crate::{instruction::{Instruction, Operand}, basic_block::BasicBlock, function::Function, program::Program};

pub struct Parser {
    tokenizer: Tokenizer,
    program: Program,
    instruction_count: isize,
}

impl Parser {
    pub fn new(input: String) -> Self {
        pub fn new(input: String) -> Self {
            Self {
                tokenizer: Tokenizer::new(input),
                program: Program::new(),
                instruction_count: 0,
            }
        }
    }

    // TODO: 
    // write base code for:
    // varDecl, funcDecl, formalParam, funcBody, computation

    fn parse_expression(&mut self) {
        let operand1 = self.parse_term();

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

    // returns an instruction line number 
    fn parse_factor(&mut self) {
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
                self.parse_fn_call();
            },
            _ => {
                panic!("ERROR: write ...");
            },
        }

    }
    
    fn parse_fn_call(&mut self) {
        self.match_token(Token::FunctionCall);
        self.match_token(Token::Function);
        // implement rest later

    }

    fn parse_if_statement(&mut self) {
        self.match_token(Token::If);
        self.parse_relation();
        self.match_token(Token::Then);
        self.parse_stats_sequence();

        if self.tokenizer.peek_token() == Token::Else {
            self.tokenizer.next_token();
            self.parse_stats_sequence();
        }

        self.match_token(Token::Fi);
    }

    fn parse_while_statement(&mut self) {
        self.match_token(Token::While);
        self.parse_relation();
        self.match_token(Token::Do);
        self.parse_stats_sequence();
        self.match_token(Token::Od);
    }

    fn parse_return_statement(&mut self) {
        self.match_token(Token::Return);
        // implement optional
        self.parse_expression();
    }

    fn parse_statement(&mut self) {
        let token = self.tokenizer.peek_token();

        match token {
            Token::Let => {
                self.parse_assignment();
            },
            Token::FunctionCall => {
                self.parse_fn_call();
            },
            Token::If => {
                self.parse_if_statement();
            },
            Token::While => {
                self.parse_while_statement();
            },
            Token::Return => {
                self.parse_return_statement();
            }
            _ => {
                panic!("ERROR: write ...");
            },
        }

    }

    fn parse_stats_sequence(&mut self) {
        loop {
            self.parse_statement();

            match self.tokenizer.next_token() {
                Token::Semicolon => (),
                _ => break,
            }
        }
    }


    fn parse_assignment(&mut self) {
        self.match_token(Token::Let);
        self.get_identifier();
        self.match_token(Token::Assignment);
        self.parse_expression();
    }

    fn parse_relation(&mut self) {
        self.parse_expression();
        self.get_operator();
        self.parse_expression();
    }

    // !!! Below are just skeleton code, just to get started. Remove anytime. !!!

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


    fn match_token(&mut self, token_to_match: Token) {
        // advances regardless of token, should always match, else syntax error
        let token = self.tokenizer.next_token();
        match token {
            token_to_match => (),
            _ => panic!("ERROR: Unexpected token, expected {:?}, instead got {:?}", token_to_match, token),
        }
    }

}

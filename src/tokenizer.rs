/// Token types representing different elements of a simple programming language.
#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Identifier(String),
    Plus,
    Minus,
    Times,
    Divide,
    Assignment,
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
    Semicolon,
    Comma,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    If,
    Fi,
    Then,
    Else,
    While,
    Do,
    Od,
    Function,
    FunctionCall,
    Return,
    Variable,
    Let,
    Call,
    Main,
    Void,
    EOF,
}

/// A tokenizer that converts a string input into a series of tokens.
pub struct Tokenizer {
    input: Vec<u8>,
    pos: usize,
}

impl Tokenizer {
    /// Create a new tokenizer with the provided input string.
    pub fn new(input: String) -> Self {
        Tokenizer {
            input: input.into_bytes(),
            pos: 0,
        }
    }

    /// Peek at the current character without advancing the tokenizer.
    pub fn peek_char(&self) -> char {
        if self.pos >= self.input.len() {
            '\0'
        } else {
            self.input[self.pos] as char
        }
    }

    /// Advance the tokenizer and return the next character.
    fn next_char(&mut self) -> char {
        let c = self.peek_char();
        self.pos += 1;
        c
    }

    /// Consume characters while the provided function returns true.
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.pos < self.input.len() && test(self.peek_char()) {
            result.push(self.next_char());
        }
        result
    }

    /// Consume all whitespace characters from the current position.
    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    /// Tokenize a sequence of digits into a Number token.
    fn tokenize_number(&mut self) -> Token {
        let number_str = self.consume_while(|c| c.is_digit(10));
        Token::Number(number_str.parse::<i32>().unwrap())
    }

    /// Tokenize an identifier or a keyword into the appropriate Token type.
    fn tokenize_identifier_or_keyword(&mut self) -> Token {
        let identifier = self.consume_while(|c| c.is_alphanumeric() || c == '_');
        match identifier.as_str() {
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "function" => Token::Function,
            "call" => Token::FunctionCall,
            "return" => Token::Return,
            "var" => Token::Variable,
            "void" => Token::Void,
            "then" => Token::Then,
            "fi" => Token::Fi,
            "do" => Token::Do,
            "od" => Token::Od,
            "let" => Token::Let,
            "call" => Token::Call,
            "main" => Token::Main,
            _ => Token::Identifier(identifier),
        }
    }

    /// Tokenize a relational operator or the assignment operator.
    fn tokenize_operator(&mut self) -> Token {
        let op = self.next_char();
        match op {
            '<' => {
                if self.peek_char() == '-' {
                    self.next_char(); // Consume '-'
                    Token::Assignment
                } else if self.peek_char() == '=' {
                    self.next_char(); // Consume '='
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.next_char(); // Consume '='
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '=' => {
                if self.peek_char() == '=' {
                    self.next_char(); // Consume '='
                    Token::Equal
                } else {
                    panic!("Unexpected character after '=': {}", self.peek_char());
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.next_char(); // Consume '='
                    Token::NotEqual
                } else {
                    panic!("Unexpected character after '!': {}", self.peek_char());
                }
            }
            _ => panic!("Unexpected operator: {}", op),
        }
    }
    
    /// Peeks the next token from the input, without advancing the tokenizer.
    pub fn peek_token(&mut self) -> Token {
        let previous_pos= self.pos;
        let token = self.next_token();
        self.pos = previous_pos;

        token
    }

    /// Retrieve the next token from the input, advancing the tokenizer.
    pub fn next_token(&mut self) -> Token {
        self.consume_whitespace();

        let token = match self.peek_char() {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Times,
            '/' => Token::Divide,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            '.' => Token::EOF,
            '0'..='9' => return self.tokenize_number(),
            '<' | '>' | '=' | '!' => return self.tokenize_operator(),
            'a'..='z' | 'A'..='Z' => return self.tokenize_identifier_or_keyword(),
            _ => panic!("Unexpected character: {}", self.peek_char()),
        };

        self.next_char();
        token
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_number_tokenization() {
//         let mut tokenizer = Tokenizer::new("123".to_string());
//         assert_eq!(tokenizer.next_token(), Token::Number(123));
//     }
//
//     #[test]
//     fn test_operators() {
//         let mut tokenizer = Tokenizer::new("<- == > >= < <= != + - * /".to_string());
//         assert_eq!(tokenizer.next_token(), Token::Assignment);
//         assert_eq!(tokenizer.next_token(), Token::Equal);
//         assert_eq!(tokenizer.next_token(), Token::Greater);
//         assert_eq!(tokenizer.next_token(), Token::GreaterEqual);
//         assert_eq!(tokenizer.next_token(), Token::Less);
//         assert_eq!(tokenizer.next_token(), Token::LessEqual);
//         assert_eq!(tokenizer.next_token(), Token::NotEqual);
//         assert_eq!(tokenizer.next_token(), Token::Plus);
//         assert_eq!(tokenizer.next_token(), Token::Minus);
//         assert_eq!(tokenizer.next_token(), Token::Times);
//         assert_eq!(tokenizer.next_token(), Token::Divide);
//     }
//
//     #[test]
//     fn test_keywords_and_identifiers() {
//         let mut tokenizer = Tokenizer::new("if else while function return var xyz".to_string());
//         assert_eq!(tokenizer.next_token(), Token::If);
//         assert_eq!(tokenizer.next_token(), Token::Else);
//         assert_eq!(tokenizer.next_token(), Token::While);
//         assert_eq!(tokenizer.next_token(), Token::Function);
//         assert_eq!(tokenizer.next_token(), Token::Return);
//         assert_eq!(tokenizer.next_token(), Token::Variable);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("xyz".to_string()));
//     }
//
//     #[test]
//     fn test_function_declaration() {
//         let mut tokenizer = Tokenizer::new("function sum(a, b) { return a + b; }".to_string());
//         assert_eq!(tokenizer.next_token(), Token::Function);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("sum".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::OpenParen);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("a".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::Comma);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("b".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::CloseParen);
//         assert_eq!(tokenizer.next_token(), Token::OpenBrace);
//         assert_eq!(tokenizer.next_token(), Token::Return);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("a".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::Plus);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("b".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::Semicolon);
//         assert_eq!(tokenizer.next_token(), Token::CloseBrace);
//     }
//
//     #[test]
//     fn test_function_declaration_with_keywords() {
//         let mut tokenizer = Tokenizer::new("void function factorial(n); { if n == 0 then return 1; else return n * call factorial(n - 1); };".to_string());
//         assert_eq!(tokenizer.next_token(), Token::Void);
//         assert_eq!(tokenizer.next_token(), Token::Function);
//         assert_eq!(
//             tokenizer.next_token(),
//             Token::Identifier("factorial".to_string())
//         );
//         assert_eq!(tokenizer.next_token(), Token::OpenParen);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("n".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::CloseParen);
//         assert_eq!(tokenizer.next_token(), Token::Semicolon);
//         assert_eq!(tokenizer.next_token(), Token::OpenBrace);
//         assert_eq!(tokenizer.next_token(), Token::If);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("n".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::Equal);
//         assert_eq!(tokenizer.next_token(), Token::Number(0));
//         assert_eq!(tokenizer.next_token(), Token::Then);
//         assert_eq!(tokenizer.next_token(), Token::Return);
//         assert_eq!(tokenizer.next_token(), Token::Number(1));
//         assert_eq!(tokenizer.next_token(), Token::Semicolon);
//         assert_eq!(tokenizer.next_token(), Token::Else);
//         assert_eq!(tokenizer.next_token(), Token::Return);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("n".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::Times);
//         assert_eq!(tokenizer.next_token(), Token::Call);
//         assert_eq!(
//             tokenizer.next_token(),
//             Token::Identifier("factorial".to_string())
//         );
//         assert_eq!(tokenizer.next_token(), Token::OpenParen);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("n".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::Minus);
//         assert_eq!(tokenizer.next_token(), Token::Number(1));
//         assert_eq!(tokenizer.next_token(), Token::CloseParen);
//         assert_eq!(tokenizer.next_token(), Token::Semicolon);
//         assert_eq!(tokenizer.next_token(), Token::CloseBrace);
//         assert_eq!(tokenizer.next_token(), Token::Semicolon);
//     }
//
//     #[test]
//     fn test_while_loop_with_assignment_and_relational() {
//         let mut tokenizer = Tokenizer::new("while x > 10 do x <- x - 1 od".to_string());
//         assert_eq!(tokenizer.next_token(), Token::While);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("x".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::Greater);
//         assert_eq!(tokenizer.next_token(), Token::Number(10));
//         assert_eq!(tokenizer.next_token(), Token::Do);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("x".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::Assignment);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("x".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::Minus);
//         assert_eq!(tokenizer.next_token(), Token::Number(1));
//         assert_eq!(tokenizer.next_token(), Token::Od);
//     }
//
//     #[test]
//     fn test_complex_nested_control_structures() {
//         let mut tokenizer = Tokenizer::new(
//             "if x < 5 then { while y >= 10 do y <- y / 2 od } else y <- 0".to_string(),
//         );
//         assert_eq!(tokenizer.next_token(), Token::If);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("x".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::Less);
//         assert_eq!(tokenizer.next_token(), Token::Number(5));
//         assert_eq!(tokenizer.next_token(), Token::Then);
//         assert_eq!(tokenizer.next_token(), Token::OpenBrace);
//         assert_eq!(tokenizer.next_token(), Token::While);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("y".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::GreaterEqual);
//         assert_eq!(tokenizer.next_token(), Token::Number(10));
//         assert_eq!(tokenizer.next_token(), Token::Do);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("y".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::Assignment);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("y".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::Divide);
//         assert_eq!(tokenizer.next_token(), Token::Number(2));
//         assert_eq!(tokenizer.next_token(), Token::Od);
//         assert_eq!(tokenizer.next_token(), Token::CloseBrace);
//         assert_eq!(tokenizer.next_token(), Token::Else);
//         assert_eq!(tokenizer.next_token(), Token::Identifier("y".to_string()));
//         assert_eq!(tokenizer.next_token(), Token::Assignment);
//         assert_eq!(tokenizer.next_token(), Token::Number(0));
//     }
// }

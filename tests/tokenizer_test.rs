mod tokenizer;
use crate::tokenizer::{Tokenizer, Token};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_tokenization() {
        let mut tokenizer = Tokenizer::new("123".to_string());
        assert_eq!(tokenizer.next_token(), Token::Number(123));
    }

    #[test]
    fn test_operators() {
        let mut tokenizer = Tokenizer::new("<- == > >= < <= != + - * /".to_string());
        assert_eq!(tokenizer.next_token(), Token::Assignment);
        assert_eq!(tokenizer.next_token(), Token::Equal);
        assert_eq!(tokenizer.next_token(), Token::Greater);
        assert_eq!(tokenizer.next_token(), Token::GreaterEqual);
        assert_eq!(tokenizer.next_token(), Token::Less);
        assert_eq!(tokenizer.next_token(), Token::LessEqual);
        assert_eq!(tokenizer.next_token(), Token::NotEqual);
        assert_eq!(tokenizer.next_token(), Token::Plus);
        assert_eq!(tokenizer.next_token(), Token::Minus);
        assert_eq!(tokenizer.next_token(), Token::Times);
        assert_eq!(tokenizer.next_token(), Token::Divide);
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let mut tokenizer = Tokenizer::new("if else while function return var xyz".to_string());
        assert_eq!(tokenizer.next_token(), Token::If);
        assert_eq!(tokenizer.next_token(), Token::Else);
        assert_eq!(tokenizer.next_token(), Token::While);
        assert_eq!(tokenizer.next_token(), Token::Function);
        assert_eq!(tokenizer.next_token(), Token::Return);
        assert_eq!(tokenizer.next_token(), Token::Variable);
        assert_eq!(tokenizer.next_token(), Token::Identifier("xyz".to_string()));
    }

    #[test]
    fn test_function_declaration() {
        let mut tokenizer = Tokenizer::new("function sum(a, b) { return a + b; }".to_string());
        assert_eq!(tokenizer.next_token(), Token::Function);
        assert_eq!(tokenizer.next_token(), Token::Identifier("sum".to_string()));
        assert_eq!(tokenizer.next_token(), Token::OpenParen);
        assert_eq!(tokenizer.next_token(), Token::Identifier("a".to_string()));
        assert_eq!(tokenizer.next_token(), Token::Comma);
        assert_eq!(tokenizer.next_token(), Token::Identifier("b".to_string()));
        assert_eq!(tokenizer.next_token(), Token::CloseParen);
        assert_eq!(tokenizer.next_token(), Token::OpenBrace);
        assert_eq!(tokenizer.next_token(), Token::Return);
        assert_eq!(tokenizer.next_token(), Token::Identifier("a".to_string()));
        assert_eq!(tokenizer.next_token(), Token::Plus);
        assert_eq!(tokenizer.next_token(), Token::Identifier("b".to_string()));
        assert_eq!(tokenizer.next_token(), Token::Semicolon);
        assert_eq!(tokenizer.next_token(), Token::CloseBrace);
    }

    #[test]
    fn test_function_declaration_with_keywords() {
        let mut tokenizer = Tokenizer::new("void function factorial(n); { if n == 0 then return 1; else return n * call factorial(n - 1); };".to_string());
        assert_eq!(tokenizer.next_token(), Token::Void);
        assert_eq!(tokenizer.next_token(), Token::Function);
        assert_eq!(
            tokenizer.next_token(),
            Token::Identifier("factorial".to_string())
        );
        assert_eq!(tokenizer.next_token(), Token::OpenParen);
        assert_eq!(tokenizer.next_token(), Token::Identifier("n".to_string()));
        assert_eq!(tokenizer.next_token(), Token::CloseParen);
        assert_eq!(tokenizer.next_token(), Token::Semicolon);
        assert_eq!(tokenizer.next_token(), Token::OpenBrace);
        assert_eq!(tokenizer.next_token(), Token::If);
        assert_eq!(tokenizer.next_token(), Token::Identifier("n".to_string()));
        assert_eq!(tokenizer.next_token(), Token::Equal);
        assert_eq!(tokenizer.next_token(), Token::Number(0));
        assert_eq!(tokenizer.next_token(), Token::Then);
        assert_eq!(tokenizer.next_token(), Token::Return);
        assert_eq!(tokenizer.next_token(), Token::Number(1));
        assert_eq!(tokenizer.next_token(), Token::Semicolon);
        assert_eq!(tokenizer.next_token(), Token::Else);
        assert_eq!(tokenizer.next_token(), Token::Return);
        assert_eq!(tokenizer.next_token(), Token::Identifier("n".to_string()));
        assert_eq!(tokenizer.next_token(), Token::Times);
        assert_eq!(tokenizer.next_token(), Token::Call);
        assert_eq!(
            tokenizer.next_token(),
            Token::Identifier("factorial".to_string())
        );
        assert_eq!(tokenizer.next_token(), Token::OpenParen);
        assert_eq!(tokenizer.next_token(), Token::Identifier("n".to_string()));
        assert_eq!(tokenizer.next_token(), Token::Minus);
        assert_eq!(tokenizer.next_token(), Token::Number(1));
        assert_eq!(tokenizer.next_token(), Token::CloseParen);
        assert_eq!(tokenizer.next_token(), Token::Semicolon);
        assert_eq!(tokenizer.next_token(), Token::CloseBrace);
        assert_eq!(tokenizer.next_token(), Token::Semicolon);
    }

    #[test]
    fn test_while_loop_with_assignment_and_relational() {
        let mut tokenizer = Tokenizer::new("while x > 10 do x <- x - 1 od".to_string());
        assert_eq!(tokenizer.next_token(), Token::While);
        assert_eq!(tokenizer.next_token(), Token::Identifier("x".to_string()));
        assert_eq!(tokenizer.next_token(), Token::Greater);
        assert_eq!(tokenizer.next_token(), Token::Number(10));
        assert_eq!(tokenizer.next_token(), Token::Do);
        assert_eq!(tokenizer.next_token(), Token::Identifier("x".to_string()));
        assert_eq!(tokenizer.next_token(), Token::Assignment);
        assert_eq!(tokenizer.next_token(), Token::Identifier("x".to_string()));
        assert_eq!(tokenizer.next_token(), Token::Minus);
        assert_eq!(tokenizer.next_token(), Token::Number(1));
        assert_eq!(tokenizer.next_token(), Token::Od);
    }

    #[test]
    fn test_complex_nested_control_structures() {
        let mut tokenizer = Tokenizer::new(
            "if x < 5 then { while y >= 10 do y <- y / 2 od } else y <- 0".to_string(),
        );
        assert_eq!(tokenizer.next_token(), Token::If);
        assert_eq!(tokenizer.next_token(), Token::Identifier("x".to_string()));
        assert_eq!(tokenizer.next_token(), Token::Less);
        assert_eq!(tokenizer.next_token(), Token::Number(5));
        assert_eq!(tokenizer.next_token(), Token::Then);
        assert_eq!(tokenizer.next_token(), Token::OpenBrace);
        assert_eq!(tokenizer.next_token(), Token::While);
        assert_eq!(tokenizer.next_token(), Token::Identifier("y".to_string()));
        assert_eq!(tokenizer.next_token(), Token::GreaterEqual);
        assert_eq!(tokenizer.next_token(), Token::Number(10));
        assert_eq!(tokenizer.next_token(), Token::Do);
        assert_eq!(tokenizer.next_token(), Token::Identifier("y".to_string()));
        assert_eq!(tokenizer.next_token(), Token::Assignment);
        assert_eq!(tokenizer.next_token(), Token::Identifier("y".to_string()));
        assert_eq!(tokenizer.next_token(), Token::Divide);
        assert_eq!(tokenizer.next_token(), Token::Number(2));
        assert_eq!(tokenizer.next_token(), Token::Od);
        assert_eq!(tokenizer.next_token(), Token::CloseBrace);
        assert_eq!(tokenizer.next_token(), Token::Else);
        assert_eq!(tokenizer.next_token(), Token::Identifier("y".to_string()));
        assert_eq!(tokenizer.next_token(), Token::Assignment);
        assert_eq!(tokenizer.next_token(), Token::Number(0));
    }
}

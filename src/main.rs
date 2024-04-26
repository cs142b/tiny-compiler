mod tokenizer;
use crate::tokenizer::{Tokenizer, Token};

fn main() {
    let mut tokenizer = Tokenizer::new("function sum(a, b) { return a + b; }".to_string());
    loop {
        let token = tokenizer.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            break;
        }

    }
}

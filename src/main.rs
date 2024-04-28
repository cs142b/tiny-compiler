mod tokenizer;
use crate::tokenizer::{Tokenizer, Token};
mod parser;

use std::io;
use parser::Parser;

fn main() {
    let mut tokenizer = Tokenizer::new("main 
var x, var y, var z; 

function addition (a,b); 
var c, var d; { 
    c <- a; 
    d <- b;
    return c + d;
};

function subtraction (a,b);
var c, var d; {
    c <- b;
    d <- a;
    return  c - d;
};

{
    let y <- inputNum();
    let z <- inputNum();
    let x <- subtraction(y,z);
    if x < 2 then
        let x <- 100;
    else
        while x < 200 do
            let x <- addition(x, 1);
        od
    fi
    outputNum(x);
}.".to_string());
    loop {
        let token = tokenizer.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            break;
        }

    }
}

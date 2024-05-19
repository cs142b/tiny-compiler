use crate::tokenizer::{Token, Tokenizer};
use crate::{instruction::{Instruction, Operation}, basic_block::BasicBlock, function::Function, program::Program, constant_block::ConstantBlock};
use petgraph::graph::NodeIndex;

pub struct Parser {
    tokenizer: Tokenizer,
    program: Program,
    line_number: isize,
    current_block: NodeIndex,
    // move this into function but used here for testing purposes
    constant_block: ConstantBlock,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let mut program = Program::new();
        let main_function = program.add_function("main".to_string(), Vec::new());
        let initial_block = NodeIndex::new(0);
        Self {
            tokenizer: Tokenizer::new(input),
            program,
            line_number: 0,
            current_block: initial_block,
            constant_block: ConstantBlock::new(),
        }
    }

    // TODO: 
    // write base code for:
    // varDecl, funcDecl, formalParam, funcBody, computation

    // Parse an expression (handles addition and subtraction)
    fn parse_expression(&mut self) -> isize {
        let line_number1 = self.parse_term();

        loop {
            match self.tokenizer.peek_token() {
                Token::Plus => {
                    self.tokenizer.next_token();
                    let line_number2 = self.parse_term();
                    return self.emit_instruction(Operation::Add(line_number1, line_number2));
                },
                Token::Minus => {
                    self.tokenizer.next_token();
                    let line_number2 = self.parse_term();
                    return self.emit_instruction(Operation::Sub(line_number1, line_number2));
                },
                _ => break,
            }
        }

        line_number1
    }

    // Parse a term (handles multiplication and division)
    fn parse_term(&mut self) -> isize {
        let line_number1 = self.parse_factor();

        loop {
            match self.tokenizer.peek_token() {
                Token::Times => {
                    self.tokenizer.next_token();
                    let line_number2 = self.parse_factor();
                    return self.emit_instruction(Operation::Mul(line_number1, line_number2));
                },
                Token::Divide => {
                    self.tokenizer.next_token();
                    let line_number2 = self.parse_factor();
                    return self.emit_instruction(Operation::Div(line_number1, line_number2));
                },
                _ => break,
            }
        }

        line_number1
    }

    // Parse a factor (handles numbers, identifiers, and parenthesized expressions)
    fn parse_factor(&mut self) -> isize {
        let token = self.tokenizer.next_token();
        match token {
            Token::Number(value) => {
                self.constant_block.get_constant(value)
            },
            Token::Identifier(name) => {
                self.program.functions[0].basic_blocks[self.current_block]
                    .get_variable(&name)
            },
            Token::OpenParen => {
                let result = self.parse_expression();
                self.match_token(Token::CloseParen);
                result
            },
            _ => panic!("Syntax error in factor"),
        }
    }

    // Parse an assignment statement
    fn parse_assignment(&mut self) {
        self.match_token(Token::Let);
        let variable_name = match self.tokenizer.next_token() {
            Token::Identifier(name) => name,
            _ => panic!("Expected identifier after 'let'"),
        };
        self.match_token(Token::Assignment);
        let expr_result = self.parse_expression();
        self.program.functions[0].basic_blocks[self.current_block]
            .add_variable(variable_name, expr_result);
    }

    // Parse a relation 
    fn parse_relation(&mut self) -> (isize, Token) {
        let line_number1 = self.parse_expression();
        let operator = self.parse_operator();
        let line_number2 = self.parse_expression();
        
        let cmp_line_number = self.emit_instruction(Operation::Cmp(line_number1, line_number2));

        (cmp_line_number, operator)
    }

    // return operator
    fn parse_operator(&mut self) -> Token {
        let operator_tokens = vec![Token::Equal, Token::NotEqual, Token::Greater, Token::GreaterEqual, Token::Less, Token::LessEqual];

        let token = self.tokenizer.next_token();

        if operator_tokens.contains(&token) {
            return token;
        } else {
            panic!("ERROR: {:?} is not a valid operator", token);
        }
    }

    // Parse an if statement
    fn parse_if_statement(&mut self) {
        self.match_token(Token::If);
        let condition = self.parse_relation(); // this returns the cmp line number and the operator
        // (which will affect what kind of branch it is)
        // so any lines of code down here, will need to be changed accordingly
        let then_block = self.program.functions[0].basic_blocks.add_node(BasicBlock::new());
        let else_block = self.program.functions[0].basic_blocks.add_node(BasicBlock::new());
        let end_block = self.program.functions[0].basic_blocks.add_node(BasicBlock::new());

        self.emit_instruction(Operation::Beq(condition, then_block.index() as isize));
        self.current_block = then_block;
        self.match_token(Token::Then);
        self.parse_stat_sequence();
        self.emit_instruction(Operation::Bra(end_block.index() as isize));

        if self.tokenizer.peek_token() == Token::Else {
            self.tokenizer.next_token();
            self.current_block = else_block;
            self.parse_stat_sequence();
            self.emit_instruction(Operation::Bra(end_block.index() as isize));
        }

        self.current_block = end_block;
        self.match_token(Token::Fi);
    }

    // Parse a while statement
    fn parse_while_statement(&mut self) {
        self.match_token(Token::While);
        let condition_block = self.current_block;
        let body_block = self.program.functions[0].basic_blocks.add_node(BasicBlock::new());
        let end_block = self.program.functions[0].basic_blocks.add_node(BasicBlock::new());

        let condition = self.parse_expression();
        self.emit_instruction(Operation::Beq(condition, end_block.index() as isize));
        self.current_block = body_block;
        self.match_token(Token::Do);
        self.parse_stat_sequence();
        self.emit_instruction(Operation::Bra(condition_block.index() as isize));
        self.current_block = end_block;
        self.match_token(Token::Od);
    }

    // Parse a sequence of statements
    fn parse_stat_sequence(&mut self) {
        loop {
            match self.tokenizer.peek_token() {
                Token::Let => self.parse_assignment(),
                Token::If => self.parse_if_statement(),
                Token::While => self.parse_while_statement(),
                Token::Return => self.parse_return_statement(),
                _ => break,
            }

            match self.tokenizer.peek_token() {
                Token::Semicolon => {
                    self.tokenizer.next_token();
                },
                _ => break,
            }
        }
    }

    // Parse a return statement
    fn parse_return_statement(&mut self) {
        self.match_token(Token::Return);
        if self.tokenizer.peek_token() != Token::Semicolon {
            let expr_result = self.parse_expression();
            self.emit_instruction(Operation::Ret(expr_result));
        } else {
            self.emit_instruction(Operation::Ret(0));
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

    // Function to emit an instruction and get the line number
    fn emit_instruction(&mut self, operation: Operation) -> isize {
        self.line_number += 1;
        let instruction = Instruction::create_instruction(self.line_number, operation);
        self.program.functions[0].basic_blocks[self.current_block].add_instruction(instruction);
        self.line_number
    }
}

//Tests
#[cfg(test)]
mod parser_tests{
    use super::*;

    #[test]
    fn test_parse_expression_add() {
        let input = "2+3.".to_string();
        let mut parser = Parser::new(input);

        let line_number = parser.parse_expression();

        // Verify that the add operation is correct
        let instructions = &parser.program.functions[0].basic_blocks[parser.current_block].instructions;

        let b = &parser.program.functions[0].basic_blocks;
        for node_index in b.node_indices() {
            let node_value = b.node_weight(node_index).unwrap();
            println!("Node Index: {:?}, Node Value: {:?}", node_index, node_value);
        }

        assert_eq!(instructions.len(), 1);
        assert_eq!(format!("{:?}", instructions[0]), "1: add (-1) (-2)");
    }

    #[test]
    fn test_parse_expression_mul() {
        let input = "2*3.".to_string();
        let mut parser = Parser::new(input);

        let line_number = parser.parse_expression();

        // Verify that the mul operation is correct
        let instructions = &parser.program.functions[0].basic_blocks[parser.current_block].instructions;
        assert_eq!(instructions.len(), 1);
        assert_eq!(format!("{:?}", instructions[0]), "1: mul (-1) (-2)");
    }

    #[test]
    fn test_parse_assignment() {
        let input = "let x <- 5.".to_string();
        let mut parser = Parser::new(input);

        parser.parse_assignment();

        // Verify that the variable x is correctly assigned
        let block = &parser.program.functions[0].basic_blocks[parser.current_block];
        let x_line_number = block.get_variable(&"x".to_string());
        assert_eq!(x_line_number, -1); // The line number for the constant 5
    }

    #[test]
    fn test_parse_if_statement() {
        let input = "if 1 then let x <- 2; fi".to_string();
        let mut parser = Parser::new(input);

        parser.parse_if_statement();

        // Verify that the if statement creates the correct basic blocks and instructions
        let blocks = &parser.program.functions[0].basic_blocks;
        let then_block = blocks.node_indices().nth(1).unwrap(); // then block
        let else_block = blocks.node_indices().nth(2).unwrap(); // else block
        let end_block = blocks.node_indices().nth(3).unwrap(); // end block

        assert_eq!(blocks[parser.current_block].instructions.len(), 0); // end block should have 0 instruction
        assert_eq!(blocks[then_block].instructions.len(), 1); // then block should have 2 instructions
        assert_eq!(blocks[else_block].instructions.len(), 0); // else block should have 0 instructions

        let then_instructions = &blocks[then_block].instructions;
        let else_instructions = &blocks[else_block].instructions;
        let end_instructions = &blocks[end_block].instructions;

        assert_eq!(format!("{:?}", then_instructions[0]), "2: bra (3)");
    }
}

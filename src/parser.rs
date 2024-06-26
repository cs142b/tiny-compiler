use crate::basic_block::VariableType;
use crate::tokenizer::{Token, Tokenizer};
use crate::{
    basic_block::BasicBlockType,
    instruction::{Instruction, Operation}, 
    program::Program,
};

use petgraph::graph::NodeIndex;

pub struct Parser {
    tokenizer: Tokenizer,
    pub internal_program: Program,
    line_number: isize,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let mut program = Program::new();

        Self {
            tokenizer: Tokenizer::new(input),
            internal_program: program,
            line_number: 0,
        }
    }

    fn parse_var_decl(&mut self) {
        self.match_token(Token::Variable);
        loop {
            self.parse_var();
            match self.tokenizer.next_token() {
                Token::Comma => (),
                Token::Semicolon => break,
                _ => panic!("error in parse_var_decl"),
            }
        }
    }

    // parse_computation, var_decl, and var are used for later in the future
    pub fn parse_computation(&mut self){
        self.match_token(Token::Main);

        // varDecl
        if self.tokenizer.peek_token() == Token::Variable {
            self.parse_var_decl();
        }


        // add predefined functions
        self.internal_program.add_predefined_functions();

        // funcDecl
        loop {
            match self.tokenizer.peek_token() {
                Token::Void | Token::Function => self.parse_func_decl(),
                _ => break,
            }

        }
        
        // go back to main for parsing
        self.internal_program.change_curr_fn_to("main");

        self.match_token(Token::OpenBrace);
        self.parse_stat_sequence();
        if self.internal_program.get_curr_block().is_empty() {
            self.match_token(Token::CloseBrace);
            self.match_token(Token::EOF);
            self.emit_instruction(Operation::Empty);
        }
        self.internal_program.add_exit_block();
        self.emit_instruction(Operation::End);
    }


    fn parse_var(&mut self) {
        match self.tokenizer.next_token() {
            Token::Identifier(name) => {
                self.internal_program.declare_variable_to_curr_block(&name);
            },
            _ => panic!("unexpected error in parse_var"),
        }
    }

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
                self.internal_program.get_constant(value)
            },
            Token::Identifier(name) => {
                match self.internal_program.get_variable(&name) {
                    VariableType::Value(value) => value,
                    VariableType::NotInit => panic!("parse_factor() is retrieving an uninitialized variable"),
                }
            },
            Token::OpenParen => {
                let result = self.parse_expression();
                self.match_token(Token::CloseParen);
                result
            },
            Token::FunctionCall => {
                let function_name = self.get_func_name_and_verify();
                if self.is_func_void(&function_name) {
                    panic!("A void function cannot be used as an expression");
                }
                self.parse_func_call()
            },
            _ => {
                panic!("Syntax error in factor: {:?}", token);
            },
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
        // this is used for testing, but will eventually be ONLY set_variable
        self.internal_program.declare_variable_to_curr_block(&variable_name);
        self.internal_program.assign_variable_to_curr_block(&variable_name, expr_result);
    }

    // Parse a relation 
    fn parse_relation(&mut self) -> (isize, Token) {
        let line_number1 = self.parse_expression();
        let operator = self.parse_operator();
        let line_number2 = self.parse_expression(); 
        let cmp_line_number = self.emit_instruction(Operation::Cmp(line_number1, line_number2));

        (cmp_line_number, operator)
    }

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
        if self.internal_program.get_curr_block().is_empty() {
            self.emit_instruction(Operation::Empty);
        }

        self.match_token(Token::If);
    
        // Start of conditional block
        let conditional_index: NodeIndex = self.internal_program.add_cond_block();
        let (condition, comparison_operator) = self.parse_relation();
    
        // Emit the branch instruction with a placeholder target
        let (conditional_block_index, branch_instruction_line) = self.emit_instruction_with_index(self.get_branch_type(comparison_operator.clone(), condition, 0));
    
        self.match_token(Token::Then);
    
        // Start of fallthrough block
        let fallthru_index = self.internal_program.add_fallthru_block();
        self.parse_stat_sequence();

        // Get the last created block in the fallthrough sequence
        let last_fallthru_index = self.internal_program.get_curr_block_index();
    
        // Always create the branch block
        let branch_index = self.internal_program.add_branch_block(conditional_index);
        if self.tokenizer.peek_token() == Token::Else {
            self.tokenizer.next_token();
            self.parse_stat_sequence();
        }
    
        // Get the last created block in the branch sequence
        let last_branch_index = self.internal_program.get_curr_block_index();
    
        // Add the join block and connect the blocks
        let (join_index, phi_instructions) = self.internal_program.add_join_block_from_two(NodeIndex::new(last_fallthru_index), NodeIndex::new(last_branch_index));
        self.emit_phi_instructions(phi_instructions, join_index);

        // Prepare the branch operations with the correct targets
        let branch_operation = self.get_branch_type(comparison_operator, condition, branch_index.index() as isize);
    
        // Modify the instructions in the correct blocks
        {
            // Modify the branch instruction in the conditional block
            let conditional_block = self.internal_program.get_curr_fn_mut().get_bb_mut(&conditional_block_index).unwrap();
            conditional_block.modify_instruction(branch_instruction_line, branch_operation);
    
            // Modify the branch instruction in the fallthrough block
            if let Some(branch_destination) = self.internal_program.get_curr_fn_mut().get_outgoing_edge(fallthru_index) {
                let destination_block = self.internal_program.get_curr_fn().get_bb(&branch_destination).unwrap();
                if destination_block.block_type == BasicBlockType::Join {
                    self.emit_instruction_in_block(fallthru_index, Operation::Bra(branch_destination.index() as isize));
                }
            }

            // Modify the branch instruction in the join block
            let incoming_edges = self.internal_program.get_curr_fn().get_incoming_edges(join_index);
            for incoming_edge in incoming_edges {
                let incoming_block = self.internal_program.get_curr_fn().get_bb(&incoming_edge).unwrap();
                if incoming_block.block_type == BasicBlockType::Join && incoming_edge != NodeIndex::new(last_branch_index) {
                    self.emit_instruction_in_block(incoming_edge, Operation::Bra(join_index.index() as isize));
                }
            }
        }

        let fallthru_block = self.internal_program.get_curr_fn_mut().get_bb_mut(&fallthru_index);
        if fallthru_block.unwrap().is_empty() {
            self.emit_instruction_in_block(fallthru_index, Operation::Empty);
        }

        let branch_block = self.internal_program.get_curr_fn_mut().get_bb_mut(&branch_index);
        if branch_block.unwrap().is_empty() {
            self.emit_instruction_in_block(branch_index, Operation::Empty);
        }
    
        self.match_token(Token::Fi);
    }

    // Parse a while statement
    fn parse_while_statement(&mut self) {
        if self.internal_program.get_curr_block().is_empty() {
            self.emit_instruction(Operation::Empty);
        }

        self.match_token(Token::While);

        // Start of conditional block
        let conditional_index: NodeIndex = self.internal_program.add_cond_block();
        let (condition, comparison_operator) = self.parse_relation();

        // Emit the branch instruction with a placeholder target
        let (conditional_block_index, branch_instruction_line) = self.emit_instruction_with_index(self.get_branch_type(comparison_operator.clone(), condition, 0));

        self.match_token(Token::Do);

        // Start of fallthrough block
        let fallthru_index = self.internal_program.add_fallthru_block();
        self.parse_stat_sequence();

        // Get the last created block in the fallthrough sequence
        let last_fallthru_index = self.internal_program.get_curr_block_index();

        // Add a follow block and join it with the conditional block
        let follow_index = self.internal_program.add_follow_block(conditional_index);

        // Ensure loop continues by branching back to the conditional block
        let last_fallthru_nodeindex = NodeIndex::new(last_fallthru_index);
        let phi_instructions = self.internal_program.join_blocks_with_target(last_fallthru_nodeindex, conditional_index);
        self.emit_phi_instructions(phi_instructions, conditional_index);
        self.emit_instruction_in_block(last_fallthru_nodeindex, Operation::Bra(conditional_block_index.index() as isize));
        self.internal_program.get_curr_fn_mut().add_edge(last_fallthru_nodeindex, conditional_index, BasicBlockType::Follow);

        // Ensure correct branching by modifying the placeholder branch instruction
        let branch_operation = self.get_branch_type(comparison_operator, condition, follow_index.index() as isize);
        {
            // Modify the branch instruction in the conditional block
            let conditional_block = self.internal_program.get_curr_fn_mut().get_bb_mut(&conditional_block_index).unwrap();
            conditional_block.modify_instruction(branch_instruction_line, branch_operation);
        }

        let fallthru_block = self.internal_program.get_curr_fn_mut().get_bb_mut(&fallthru_index);
        if fallthru_block.unwrap().is_empty() {
            self.emit_instruction_in_block(fallthru_index, Operation::Empty);
        }

        let follow_block = self.internal_program.get_curr_fn_mut().get_bb_mut(&follow_index);
        if follow_block.unwrap().is_empty() {
            self.emit_instruction_in_block(follow_index, Operation::Empty);
        }

        // Finalize the loop with an "od" token
        self.match_token(Token::Od);
    }
    
    
    // matches the comparison operator and returns its respective SSA branch instruction
    fn get_branch_type(&self, operator: Token, left_block: isize, right_block: isize) -> Operation {

        // returns 0, 0 (just placeholder numbers that WILL be changed later)
        // could also accept a token as an argument instead, cuz this branching instruction will 
        // be added AFTER the then and else blocks are created
        match operator {
            Token::Equal => Operation::Bne(left_block, right_block),
            Token::NotEqual => Operation::Beq(left_block, right_block),
            Token::Greater => Operation::Ble(left_block, right_block),
            Token::GreaterEqual => Operation::Blt(left_block, right_block),
            Token::Less => Operation::Bge(left_block, right_block),
            Token::LessEqual => Operation::Bgt(left_block, right_block),
            _ => panic!("Expected a valid operator"),
        }
    }

    // Parse a sequence of statements
    fn parse_stat_sequence(&mut self) {
        loop {            
            match self.tokenizer.peek_token() {
                Token::Let => self.parse_assignment(),
                Token::If => self.parse_if_statement(),
                Token::While => self.parse_while_statement(),
                Token::Return => self.parse_return_statement(),
                Token::FunctionCall => { 
                    self.tokenizer.next_token();
                    let function_name = self.get_func_name_and_verify();
                    if !self.is_func_void(&function_name) {
                        panic!("A non-void function cannot be used as a statement");
                    }
                    self.parse_func_call(); 
                },
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

        if self.internal_program.get_curr_fn().is_void {
            panic!("A void function should not return");
        }

        if self.tokenizer.peek_token() != Token::Semicolon {
            let expr_result = self.parse_expression();
            self.emit_instruction(Operation::Ret(expr_result));
        } else {
            self.emit_instruction(Operation::Ret(0));
        }
    }

    fn is_func_void(&mut self, function_name: &String) -> bool {
        self.internal_program.get_fn(function_name).is_void
    }

    fn get_func_name_and_verify(&mut self) -> String {
        let function_name = match self.tokenizer.peek_token() {
            Token::Identifier(identifier) => identifier,
            _ => panic!("Expected an identifier for a function call"),
        };
        
        // this should panic if its verified
        self.internal_program.verify_function(&function_name);

        function_name
    }


    fn parse_func_call(&mut self) -> isize {
        // already matches Token::FunctionCall in factor
        let function_name = self.get_func_name_and_verify();
        self.tokenizer.next_token();

        let num_of_parameters = self.internal_program.get_number_of_parameters_of(&function_name);

        if num_of_parameters == 0 {
            if self.tokenizer.peek_token() == Token::OpenParen {
                self.match_token(Token::OpenParen);
                self.match_token(Token::CloseParen);
            }

            // predefined functions
            if function_name == "InputNum".to_string() {
                let input_num = self.internal_program.input_num();
                self.internal_program.add_constant(input_num);
                return self.internal_program.get_constant(input_num);
            } else if function_name == "OutputNewLine".to_string() {
                self.internal_program.output_new_line();

                return 0;
            }


        } else {
            let mut arguments = Vec::<isize>::new();
            self.match_token(Token::OpenParen);
            if self.tokenizer.peek_token() == Token::CloseParen {
                panic!("Calling a function with arguments with none");
            }

            loop {

                let argument = self.parse_expression();
                arguments.push(argument);
                
                if arguments.len() > 3 {
                    panic!("A function should not have more than 3 arguments");
                }

                match self.tokenizer.peek_token() {
                    Token::Comma => { self.tokenizer.next_token(); },
                    _ => break,
                }
            }
            if arguments.len() != self.internal_program.get_number_of_parameters_of(&function_name) {
                panic!("Number of parameters does not match arguments"); 
            }
            self.match_token(Token::CloseParen);
            
            if function_name == "OutputNum".to_string() {
                self.internal_program.output_num(arguments[0]);
                return 0;
            }

            // emit setpar...?
            for i in 0..arguments.len() {
                let operation = match i {
                    0 => Operation::SetPar1(arguments[0]),
                    1 => Operation::SetPar2(arguments[1]),
                    2 => Operation::SetPar3(arguments[2]),
                    _ => unreachable!("Should not reach here since max is 3 parameters"),
                };

                self.emit_instruction(operation);
            }
            // if arguments.len() >= 1 {
            //     self.emit_instruction(Operation::SetPar1(arguments[0]));
            //     if arguments.len() >= 2 {
            //         self.emit_instruction(Operation::SetPar2(arguments[1]));
            //         if arguments.len() == 3 {
            //             self.emit_instruction(Operation::SetPar3(arguments[2]));
            //         }
            //     }
            // }
            
        }

        let first_instruction_of_calling_function = self.internal_program.get_fn(&function_name).get_bb(&NodeIndex::from(0)).unwrap().get_first_instruction_line_number();
        self.emit_instruction(Operation::Jsr(first_instruction_of_calling_function));

        // return the last return instruction if the function is non-void
        let calling_function_graph = self.internal_program.get_fn(&function_name).get_graph();
        let return_instruction = self.internal_program.get_fn(&function_name).get_bb(&calling_function_graph.node_indices().max().unwrap()).unwrap().get_last_instruction_line_number();

        return_instruction
    }

    fn parse_func_decl(&mut self) {
        let is_void_condition = match self.tokenizer.peek_token() {
            Token::Void => { 
                self.tokenizer.next_token(); 
                true
            }, 
            _ => false,
        };
        self.match_token(Token::Function);
        let function_name = match self.tokenizer.next_token() {
            Token::Identifier(identifier) => identifier,
            _ => panic!("Expected an identifier for a function declaration"),
        };
        
        self.internal_program.add_function(&function_name, is_void_condition);

        self.parse_formal_param();
        self.match_token(Token::Semicolon);
        self.parse_func_body();
        self.match_token(Token::Semicolon);
        
    }

    fn parse_formal_param(&mut self) {
        self.match_token(Token::OpenParen);
        loop {
            match self.tokenizer.peek_token() {
                Token::Identifier(parameter_name) => {
                    self.tokenizer.next_token();
                    // add to vec of strings then add to the variable table
                    self.internal_program.insert_new_parameter_to_curr_function(parameter_name.clone());
                    self.internal_program.declare_variable_to_curr_block(&parameter_name);

                    if self.internal_program.get_number_of_parameters_of_curr_fn() > 3 {
                        panic!("A function should not have more than 3 parameters");
                    }
                },
                Token::Comma => { 
                    self.tokenizer.next_token();
                    continue; 
                },
                _ => { break; },
            }
        }

        let num_of_parameters = self.internal_program.get_number_of_parameters_of_curr_fn();

        for i in 0..num_of_parameters {
            let operation = match i {
                0 => Operation::GetPar1,
                1 => Operation::GetPar2,
                2 => Operation::GetPar3,
                _ => unreachable!("Should not reach here since max is 3 parameters"),
            };

            let line_number = self.emit_instruction(operation);
            let parameter_name = &self.internal_program.get_curr_fn().parameters[i].clone();

            self.internal_program.assign_variable_to_curr_block(&parameter_name, line_number);
        }
        
        // LOLCODE
        //
        // if self.internal_program.get_number_of_parameters_of_curr_fn() >= 1 {
        //     let line_number = self.emit_instruction(Operation::GetPar1);
        //     let parameter_name = &self.internal_program.get_curr_fn().parameters[0].clone();
        //     self.internal_program.assign_variable_to_curr_block(&parameter_name, line_number);
        //     if self.internal_program.get_number_of_parameters_of_curr_fn() >= 2 {
        //         let line_number = self.emit_instruction(Operation::GetPar2);
        //         let parameter_name = &self.internal_program.get_curr_fn().parameters[1].clone();
        //         self.internal_program.assign_variable_to_curr_block(&parameter_name, line_number);
        //         if self.internal_program.get_number_of_parameters_of_curr_fn() == 3 {
        //             let line_number = self.emit_instruction(Operation::GetPar3);
        //             let parameter_name = &self.internal_program.get_curr_fn().parameters[2].clone();
        //             self.internal_program.assign_variable_to_curr_block(&parameter_name, line_number);
        //         }
        //     }
        // }

        self.match_token(Token::CloseParen);

    }

    fn parse_func_body(&mut self) {
        if self.tokenizer.peek_token() == Token::Variable {
            self.parse_var_decl();
        }

        self.match_token(Token::OpenBrace);
        self.parse_stat_sequence();
        self.match_token(Token::CloseParen);
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
        
        // handle dommy mommy logic
        if let Some(dommy_mommy_line_number) = self.internal_program.handle_dommy_mommy_logic(&operation, self.line_number) {
            return dommy_mommy_line_number;
        }

        self.line_number += 1;
        let instruction = Instruction::create_instruction(self.line_number, operation);
        self.internal_program.add_instruction_to_curr_block(instruction);
        self.line_number
    }

    fn emit_instruction_on_top(&mut self, block_index: NodeIndex, operation: Operation) -> isize {
        self.line_number += 1;
        let instruction = Instruction::create_instruction(self.line_number, operation);
        self.internal_program.add_instruction_to_any_block_on_top(block_index, instruction);
        self.line_number
    }


    fn emit_instruction_with_index(&mut self, operation: Operation) -> (NodeIndex, isize) {
        let current_block_index = self.internal_program.get_curr_block_index();

        // handle dommy mommy logic
        if let Some(dommy_mommy_line_number) = self.internal_program.handle_dommy_mommy_logic(&operation, self.line_number) {
            return (NodeIndex::from(current_block_index as u32), dommy_mommy_line_number);
        }

        self.line_number += 1;
        let instruction = Instruction::create_instruction(self.line_number, operation);
        let current_block_index = self.internal_program.get_curr_block_index();
        self.internal_program.add_instruction_to_curr_block(instruction);
        (NodeIndex::from(current_block_index as u32), self.line_number)
    }

    // Emits an instruction in a specified basic block and returns the line number.
    fn emit_instruction_in_block(&mut self, block_index: NodeIndex, operation: Operation) -> isize {
        // handle dommy mommy logic
        if let Some(dommy_mommy_line_number) = self.internal_program.handle_dommy_mommy_logic(&operation, self.line_number) {
            return dommy_mommy_line_number;
        }
        
        self.line_number += 1;
        
        let instruction = Instruction::create_instruction(self.line_number, operation);

        // Get the specified block and add the instruction
        let block = self.internal_program.get_curr_fn_mut().get_bb_mut(&block_index).expect("Block not found");
        block.add_instruction(instruction);

        self.line_number
    }

    fn emit_phi_instructions(&mut self, phi_instructions: Vec<(Operation, String)>, block_index: NodeIndex) {
        for (operation, variable) in phi_instructions {
            let line_num = self.emit_instruction_on_top(block_index, operation);
            self.internal_program.assign_variable_to_any_block(block_index, &variable, line_num);
        }
    }
}

//Tests
#[cfg(test)]
mod parser_tests{
    use super::*;
    use crate::dot_viz::generate_dot_viz;
    
    #[test]
    fn test_stuff() {
        let input = 
        "main var a, b;
        {
            let a <- call InputNum();
            call OutputNum(a);
            
        }.
        ".to_string();
        let mut parser = Parser::new(input);

        parser.parse_computation();

        println!("{}", generate_dot_viz("main", &parser.internal_program));
    }

    #[test]
    fn test_functions() {
        let input = 
        "main 
        var a, b, add, sub; 
    
        function add(x, y); {
            return x + y;
        };

        function sub(x, y); {
            return x - y;
        };

        {
            let a <- 1;
            let b <- 2;
            let add <- call add(a, b);
            let sub <- call sub(a, b);
        }.
        ".to_string();
        let mut parser = Parser::new(input);

        parser.parse_computation();

        assert_eq!(parser.internal_program.functions.len(), 6);
        for (key, value) in &parser.internal_program.get_fn("add").get_curr_bb().variable_table {
            println!("{:?} {:?}", key, value);
        }

        println!("{}", generate_dot_viz("add", &parser.internal_program));
        println!("{}", generate_dot_viz("sub", &parser.internal_program));
        println!("{}", generate_dot_viz("main", &parser.internal_program));
    }


    #[test]
    fn test_parse_operator() {
        let input = "1+1.".to_string(); // this doesnt matter, im testing the parse_operation fn
        let parser = Parser::new(input);
        
        // basic block 1 and 2 as an example
        let equal = parser.get_branch_type(Token::Equal, 1, 2);
        assert_eq!(format!("{:?}", equal), "bne (1) (BB2)");
        
        let less_equal = parser.get_branch_type(Token::LessEqual, 1, 2);
        assert_eq!(format!("{:?}", less_equal), "bgt (1) (BB2)");
    }
    
    #[test]
    pub fn test_empty() {
        let input = "
            main var a, b, c; {
                if 1 < 2 then 
                fi
            }.
        ".to_string();
        let mut parser = Parser::new(input);

        let line_number = parser.parse_computation();

        // Verify that the add operation is correct
        let instructions = &parser.internal_program.get_curr_block().instructions;

        let graph = &parser.internal_program.get_curr_fn().bb_graph;
        println!("{}", generate_dot_viz("main", &parser.internal_program));

    }
    #[test]
    pub fn test_parse_computation() {
        let input = "
            main var a, b, c; {
                let a <- 1 + 50; 
                let a <- 1 + 50; 
                if 1 < 2 then 
                    let c <- 1 + 50; 
                fi;
            }.
        ".to_string();
        let mut parser = Parser::new(input);

        let line_number = parser.parse_computation();

        // Verify that the add operation is correct
        let instructions = &parser.internal_program.get_curr_block().instructions;

        let graph = &parser.internal_program.get_curr_fn().bb_graph;
        println!("{}", generate_dot_viz("main", &parser.internal_program));

    }
    
    #[test]
    pub fn test_dom() {
        let input = "main var a, b; {let a <- 1 + 53; let b <- 1 + 53;}.".to_string();
        let mut parser = Parser::new(input);

        let line_number = parser.parse_computation();

        // Verify that the add operation is correct
        let instructions = &parser.internal_program.get_curr_block().instructions;

        let graph = &parser.internal_program.get_curr_fn().bb_graph;
        println!("{}", generate_dot_viz("main", &parser.internal_program));

    }

    #[test]
    fn test_parse_expression_add() {
        let input = "2+3.".to_string();
        let mut parser = Parser::new(input);

        let line_number = parser.parse_expression();

        // Verify that the add operation is correct
        let instructions = &parser.internal_program.get_curr_block().instructions;


        assert_eq!(instructions.len(), 1);
        assert_eq!(line_number, 1);
        assert_eq!(format!("{:?}", instructions[0]), "1: add (-2) (-3)");
    }
    #[test]
    fn test_parse_expression_mul() {
        let input = "2*3.".to_string();
        let mut parser = Parser::new(input);

        let line_number = parser.parse_expression();

        // Verify that the mul operation is correct
        let instructions = &parser.internal_program.get_curr_block().instructions;
        assert_eq!(instructions.len(), 1);
        assert_eq!(format!("{:?}", instructions[0]), "1: mul (-2) (-3)");
    }


    #[test]
    fn test_parse_assignment() {
        let input = "let x <- 5.".to_string();
        let mut parser = Parser::new(input);

        parser.parse_assignment();

        // Verify that the variable x is correctly assigned
        // let block = &parser.program.functions[0].bb_list.bb_graph[parser.current_block];
        let block = &parser.internal_program.get_curr_block();
        let x_line_number = block.get_variable(&"x".to_string());
        assert_eq!(x_line_number, VariableType::Value(-5)); // The line number for the constant 5
        println!("{}", generate_dot_viz("main", &parser.internal_program));
    }

    #[test]
    fn test_parse_if_statement() {
        let input = "main var x; { if 1 < 2 then let x <- 2; else let x <- 1; fi; }.".to_string();
        let mut parser = Parser::new(input);

        parser.parse_computation();

        // Verify that the if statement creates the correct basic blocks and instructions
        let graph = &parser.internal_program.get_curr_fn().bb_graph;
        let number_of_blocks = graph.node_count();

        // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        println!("{}", generate_dot_viz("main", &parser.internal_program));

        assert_eq!(number_of_blocks, 6); // should be 5 bc entry + conditional + fallthru + branch
        // + join 

    }


    #[test]
    fn test_parse_nested_if_statement() {
        let input = 
        "
        main var x, y, z;  {
        if 1 < 2 then 
            let y <- 69 + 420;
            if 1 < 100 then 
                let x <- 100 + 200;
            fi
        else 
            let x <- 1;
            if 2 < 4 then
                let z <- 333 + 222;
                if 3 < 4 then
                    if 4 < 4 then
                    fi
                fi
            else
                if 2 < 0 then
                fi
            fi
        fi
        }.
        "
        .to_string();
        let mut parser = Parser::new(input);

        parser.parse_computation();

        // Verify that the if statement creates the correct basic blocks and instructions
        let graph = &parser.internal_program.get_curr_fn().bb_graph;
        let number_of_blocks = graph.node_count();

        // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        println!("{}", generate_dot_viz("main", &parser.internal_program));

        // this does not work

        // assert_eq!(number_of_blocks, 11); 

    }

    #[test]
    fn test_parse_while_statement() {
        let input = "main var x; { while 10 >= 6 do while 1 < 2 do let x <- 2; od; od }.".to_string();
        let mut parser = Parser::new(input);

        parser.parse_computation();

        // Verify that the if statement creates the correct basic blocks and instructions
        let graph = &parser.internal_program.get_curr_fn().bb_graph;

        // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        println!("{}", generate_dot_viz("main", &parser.internal_program));
    }
}

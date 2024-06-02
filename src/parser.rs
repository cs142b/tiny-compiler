// use crate::dominator_tree::DominatorTree; 
// use crate::basic_block::VariableType;
// use crate::tokenizer::{Token, Tokenizer};
// use crate::{
//     instruction::{Instruction, Operation}, 
//     program::Program,
// };
//
// pub struct Parser {
//     tokenizer: Tokenizer,
//     internal_program: Program,
//     internal_dtree: DominatorTree, 
//     line_number: isize,
// }
//
// impl Parser {
//     pub fn new(input: String) -> Self {
//         let mut program = Program::new();
//         program.add_function("main".to_string(), Vec::new());
//
//         Self {
//             tokenizer: Tokenizer::new(input),
//             internal_program: Program::new(),
//             internal_dtree: DominatorTree::new(), 
//             line_number: 0,
//         }
//     }
//
//     fn parse_var_decl(&mut self) {
//         self.match_token(Token::Variable);
//         loop {
//             self.parse_var();
//             match self.tokenizer.next_token() {
//                 Token::Comma => (),
//                 Token::Semicolon => break,
//                 _ => panic!("error in parse_var_decl"),
//             }
//         }
//     }
//
//     // parse_computation, var_decl, and var are used for later in the future
//     fn parse_computation(&mut self){
//         /*
//         self.match_token(Token::Main);
//         This should not be here
//         There should be another function called parse_main that should return this basic block
//
//
//         */
//
//
//
//         // varDecl
//         if self.tokenizer.peek_token() == Token::Variable {
//             self.parse_var_decl();
//         }
//
//         // funcDecl can be done later, ^^ varDecl and funcDecl can be turned into a match later
//
//         self.match_token(Token::OpenBrace);
//         self.parse_stat_sequence();
//         self.match_token(Token::CloseBrace);
//         self.match_token(Token::EOF);
//     }
//
//     
//
//     fn parse_var(&mut self) {
//         match self.tokenizer.next_token() {
//             Token::Identifier(name) => {
//                 self.internal_program.add_uninitialized_variable_to_curr_block(&name);
//             },
//             _ => panic!("unexpected error in parse_var"),
//         }
//     }
//
//     // Parse an expression (handles addition and subtraction)
//     fn parse_expression(&mut self) -> isize {
//         let line_number1 = self.parse_term();
//
//         loop {
//             match self.tokenizer.peek_token() {
//                 Token::Plus => {
//                     self.tokenizer.next_token();
//                     let line_number2 = self.parse_term();
//                     return self.emit_instruction(Operation::Add(line_number1, line_number2));
//                 },
//                 Token::Minus => {
//                     self.tokenizer.next_token();
//                     let line_number2 = self.parse_term();
//                     return self.emit_instruction(Operation::Sub(line_number1, line_number2));
//                 },
//                 _ => break,
//             }
//         }
//
//         line_number1
//     }
//
//     // Parse a term (handles multiplication and division)
//     fn parse_term(&mut self) -> isize {
//         let line_number1 = self.parse_factor();
//
//         loop {
//             match self.tokenizer.peek_token() {
//                 Token::Times => {
//                     self.tokenizer.next_token();
//                     let line_number2 = self.parse_factor();
//                     return self.emit_instruction(Operation::Mul(line_number1, line_number2));
//                 },
//                 Token::Divide => {
//                     self.tokenizer.next_token();
//                     let line_number2 = self.parse_factor();
//                     return self.emit_instruction(Operation::Div(line_number1, line_number2));
//                 },
//                 _ => break,
//             }
//         }
//
//         line_number1
//     }
//
//     // Parse a factor (handles numbers, identifiers, and parenthesized expressions)
//     fn parse_factor(&mut self) -> isize {
//         let token = self.tokenizer.next_token();
//         match token {
//             Token::Number(value) => {
//                 self.internal_program.get_constant(value)
//             },
//             Token::Identifier(name) => {
//                 match self.internal_program.get_variable(&name) {
//                     VariableType::NotPhi(value) => value,
//                     VariableType::Phi(value1, value2) => {
//                         self.emit_instruction(Operation::Phi(value1, value2))
//                     },
//                 }
//             },
//             Token::OpenParen => {
//                 let result = self.parse_expression();
//                 self.match_token(Token::CloseParen);
//                 result
//             },
//             _ => panic!("Syntax error in factor"),
//         }
//     }
//
//     // Parse an assignment statement
//     fn parse_assignment(&mut self) {
//         self.match_token(Token::Let);
//         let variable_name = match self.tokenizer.next_token() {
//             Token::Identifier(name) => name,
//             _ => panic!("Expected identifier after 'let'"),
//         };
//         self.match_token(Token::Assignment);
//         let expr_result = self.parse_expression();
//         // this is used for testing, but will eventually be ONLY set_variable
//         self.internal_program.add_uninitialized_variable_to_curr_block(&variable_name);
//         self.internal_program.assign_variable_to_curr_block(&variable_name, VariableType::NotPhi(expr_result));
//     }
//
//     // Parse a relation 
//     fn parse_relation(&mut self) -> (isize, Token) {
//         let line_number1 = self.parse_expression();
//         let operator = self.parse_operator();
//         let line_number2 = self.parse_expression(); 
//         let cmp_line_number = self.emit_instruction(Operation::Cmp(line_number1, line_number2));
//
//         (cmp_line_number, operator)
//     }
//
//     fn parse_operator(&mut self) -> Token {
//         let operator_tokens = vec![Token::Equal, Token::NotEqual, Token::Greater, Token::GreaterEqual, Token::Less, Token::LessEqual];
//
//         let token = self.tokenizer.next_token();
//
//         if operator_tokens.contains(&token) {
//             return token;
//         } else {
//             panic!("ERROR: {:?} is not a valid operator", token);
//         }
//     }
//
//     // Parse an if statement
//     /*
//     fn parse_if_statement(&mut self) {
//         self.match_token(Token::If);
//         let (condition, comparison_operator) = self.parse_relation();
//         // let else_block = self.program.functions[0].bb_list.bb_graph.add_node(BasicBlock::new());
//         // let end_block = self.program.functions[0].bb_list.bb_graph.add_node(BasicBlock::new());
//         
//         // i can assume since its an if statement, it will branch + 2
//         let branch_index = self.program.functions[0].get_current_index().index() as isize + 2;
//         self.emit_instruction(self.get_branch_type(comparison_operator, condition, branch_index));
//         // self.program.functions[0].bb_list.add_edge(self.current_block, then_block)
//         // self.program.functions[0].bb_list.add_edge(self.current_block, else_block);
//
//         self.match_token(Token::Then);
//         let then_block = self.program.functions[0].add_fall_thru_block(BasicBlockType::FallThrough);
//         
//         // this shit is not real, i have to calculate the end_index like branch_index cuz im forced
//         // to go in one direction 
//         // and its 2am and i cant think straight
//         self.parse_stat_sequence();
//         self.emit_instruction(Operation::Bra(end_block.index() as isize));
//         self.program.functions[0].bb_list.add_edge(self.current_block, end_block);
//
//         if self.tokenizer.peek_token() == Token::Else {
//             self.tokenizer.next_token();
//             self.current_block = else_block;
//             self.parse_stat_sequence();
//             self.emit_instruction(Operation::Bra(end_block.index() as isize));
//             self.program.functions[0].bb_list.add_edge(self.current_block, end_block);
//         }
//
//         self.current_block = end_block;
//         self.match_token(Token::Fi);
//     }*/
//
//     // Parse a while statement
//     /*
//     fn parse_while_statement(&mut self) {
//         self.match_token(Token::While);
//         let condition_block = self.current_block;
//         let body_block = self.program.functions[0].bb_list.bb_graph.add_node(BasicBlock::new());
//         let end_block = self.program.functions[0].bb_list.bb_graph.add_node(BasicBlock::new());
//
//         let (condition, comparison_operator) = self.parse_relation();
//         self.emit_instruction(self.get_branch_type(comparison_operator, condition, body_block.index() as isize));
//         self.program.functions[0].bb_list.add_edge(self.current_block, body_block);
//         self.program.functions[0].bb_list.add_edge(self.current_block, end_block);
//
//
//         self.current_block = body_block;
//         self.match_token(Token::Do);
//         self.parse_stat_sequence();
//         self.emit_instruction(Operation::Bra(condition_block.index() as isize));
//         self.program.functions[0].bb_list.add_edge(self.current_block, condition_block);
//
//         self.current_block = end_block;
//         self.match_token(Token::Od);
//     }*/
//     
//     // matches the comparison operator and returns its respective SSA branch instruction
//     fn get_branch_type(&self, operator: Token, left_block: isize, right_block: isize) -> Operation {
//
//         // returns 0, 0 (just placeholder numbers that WILL be changed later)
//         // could also accept a token as an argument instead, cuz this branching instruction will 
//         // be added AFTER the then and else blocks are created
//         match operator {
//             Token::Equal => Operation::Bne(left_block, right_block),
//             Token::NotEqual => Operation::Beq(left_block, right_block),
//             Token::Greater => Operation::Ble(left_block, right_block),
//             Token::GreaterEqual => Operation::Blt(left_block, right_block),
//             Token::Less => Operation::Bge(left_block, right_block),
//             Token::LessEqual => Operation::Bgt(left_block, right_block),
//             _ => panic!("Expected a valid operator"),
//         }
//     }
//
//     // Parse a sequence of statements
//     fn parse_stat_sequence(&mut self) {
//         loop {
//             match self.tokenizer.peek_token() {
//                 Token::Let => self.parse_assignment(),
//                 //Token::If => self.parse_if_statement(),
//                 //Token::While => self.parse_while_statement(),
//                 Token::Return => self.parse_return_statement(),
//                 _ => break,
//             }
//
//             match self.tokenizer.peek_token() {
//                 Token::Semicolon => {
//                     self.tokenizer.next_token();
//                 },
//                 _ => break,
//             }
//         }
//     }
//
//     // Parse a return statement
//     fn parse_return_statement(&mut self) {
//         self.match_token(Token::Return);
//         if self.tokenizer.peek_token() != Token::Semicolon {
//             let expr_result = self.parse_expression();
//             self.emit_instruction(Operation::Ret(expr_result));
//         } else {
//             self.emit_instruction(Operation::Ret(0));
//         }
//     }
//
//     fn match_token(&mut self, token_to_match: Token) {
//         // advances regardless of token, should always match, else syntax error
//         let token = self.tokenizer.next_token();
//         match token {
//             token_to_match => (),
//             _ => panic!("ERROR: Unexpected token, expected {:?}, instead got {:?}", token_to_match, token),
//         }
//     }
//
//     // Function to emit an instruction and get the line number
//     fn emit_instruction(&mut self, operation: Operation) -> isize {
//         self.line_number += 1;
//         let instruction = Instruction::create_instruction(self.line_number, operation);
//         self.internal_program.add_instruction_to_curr_block(instruction);
//         self.line_number
//     }
// }
//
// //Tests
// #[cfg(test)]
// mod parser_tests{
//     use super::*;
//
//     #[test]
//     fn test_parse_operator() {
//         let input = "1+1.".to_string(); // this doesnt matter, im testing the parse_operation fn
//         let parser = Parser::new(input);
//         
//         // basic block 1 and 2 as an example
//         let equal = parser.get_branch_type(Token::Equal, 1, 2);
//         assert_eq!(format!("{:?}", equal), "bne (1) (BB2)");
//         
//         let less_equal = parser.get_branch_type(Token::LessEqual, 1, 2);
//         assert_eq!(format!("{:?}", less_equal), "bgt (1) (BB2)");
//     }
//
//     /*
//     #[test]
//     fn test_parse_expression_add() {
//         let input = "2+3.".to_string();
//         let mut parser = Parser::new(input);
//
//         let line_number = parser.parse_expression();
//
//         // Verify that the add operation is correct
//         let instructions = &parser.program.functions[0].bb_list.bb_graph[parser.current_block].instructions;
//
//         let b = &parser.program.functions[0].bb_list.bb_graph;
//         for node_index in b.node_indices() {
//             let node_value = b.node_weight(node_index).unwrap();
//             println!("Node Index: {:?}, Node Value: {:?}", node_index, node_value);
//         }
//
//         assert_eq!(instructions.len(), 1);
//         assert_eq!(format!("{:?}", instructions[0]), "1: add (-2) (-3)");
//     }
//
//     #[test]
//     fn test_parse_expression_mul() {
//         let input = "2*3.".to_string();
//         let mut parser = Parser::new(input);
//
//         let line_number = parser.parse_expression();
//
//         // Verify that the mul operation is correct
//         let instructions = &parser.program.functions[0].bb_list.bb_graph[parser.current_block].instructions;
//         assert_eq!(instructions.len(), 1);
//         assert_eq!(format!("{:?}", instructions[0]), "1: mul (-2) (-3)");
//     }
//     
//     #[test]
//     fn test_parse_assignment() {
//         let input = "let x <- 5.".to_string();
//         let mut parser = Parser::new(input);
//
//         parser.parse_assignment();
//
//         // Verify that the variable x is correctly assigned
//         let block = &parser.program.functions[0].bb_list.bb_graph[parser.current_block];
//         let x_line_number = block.get_variable(&"x".to_string());
//         assert_eq!(x_line_number, -5); // The line number for the constant 5
//     }
//
//     #[test]
//     fn test_parse_if_statement() {
//         let input = "if 1 < 2 then let x <- 2; fi".to_string();
//         let mut parser = Parser::new(input);
//
//         parser.parse_if_statement();
//
//         // Verify that the if statement creates the correct basic blocks and instructions
//         let blocks = &parser.program.functions[0].bb_list.bb_graph;
//         let then_block = blocks.node_indices().nth(1).unwrap(); // then block
//         let else_block = blocks.node_indices().nth(2).unwrap(); // else block
//         let end_block = blocks.node_indices().nth(3).unwrap(); // end block
//
//         assert_eq!(blocks[parser.current_block].instructions.len(), 0); // end block should have 0 instruction
//         assert_eq!(blocks[then_block].instructions.len(), 1); // then block should have 2 instructions
//         assert_eq!(blocks[else_block].instructions.len(), 0); // else block should have 0 instructions
//
//         let then_instructions = &blocks[then_block].instructions;
//         let else_instructions = &blocks[else_block].instructions;
//         let end_instructions = &blocks[end_block].instructions;
//
//         assert_eq!(format!("{:?}", then_instructions[0]), "3: bra (BB3)");
//     }
//
//     #[test]
//     fn test_parse_while_statement() {
//         let input = "while 10 >= 6 do let x <- 2; od".to_string();
//         let mut parser = Parser::new(input);
//
//         parser.parse_while_statement();
//
//         let b = &parser.program.functions[0].bb_list.bb_graph;
//         for node_index in b.node_indices() {
//             let node_value = b.node_weight(node_index).unwrap();
//             println!("Node Index: {:?}, Node Value: {:?}", node_index, node_value);
//         }
//     }*/
// }

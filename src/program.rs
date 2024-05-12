use crate::function::Function;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

impl Program {
    pub fn new() -> Self {
        let program = Self {
            functions: Vec::new(),
        };

        program
    }

    pub fn add_function(&mut self, name: String, parameters: Vec<String>) -> &mut Function {
        let new_fn = Function::new(name, parameters);
        self.functions.push(new_fn);
        self.functions.last_mut().expect("Unexpected error in adding new function in program.")
    }
    
    //pub fn get_function(&mut self, name: &String) -> &mut Function {
         
    //}

}

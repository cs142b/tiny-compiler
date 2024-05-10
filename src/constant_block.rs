use std::collections::HashMap;

pub struct ConstantBlock {
    constant_table: HashMap<isize, isize>,
    line_number: isize,
}

impl ConstantBlock {
    /// Creates and returns a new constant block
    pub fn new() -> Self {
        ConstantBlock {
            constant_table: HashMap::<isize, isize>::new(),
            line_number: -1,
        }
    }

    /// Adds a constant into the table
    pub fn add_constant(&mut self, constant: isize) {
        self.constant_table.insert(constant, self.line_number);
        self.line_number -=1;
    }

    /// Gets a constant from the table and returns the line number
    pub fn get_constant(&self, constant: isize) -> isize {
        match self.constant_table.get(isize) {
            Some(constant) => constant,
            None => (),
        }
    }
}

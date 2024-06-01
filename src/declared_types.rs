use std::ptr::NonNull;
use petgraph::graph::{Graph, UnGraph, DiGraph};
use crate::basic_block::BasicBlock;
type variables = u8; 
type LiveSetGraph = UnGraph<variables, variables>; 
pub type var_map = std::collections::HashMap<String, Option<crate::basic_block::VariableType>>;


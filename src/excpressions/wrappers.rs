use std::collections::HashMap;

use crate::lexer::token;

use super::expressions::Stantement;

#[derive(Debug)]
pub struct SwarmDescriptor {
    pub name:String,
    pub parameters: Vec<String>,
    pub io_pipes:Vec<String>,
    pub internal_pipes:Vec<String>,
    pub instructions: Vec<token::Token>,
    pub corutines: HashMap<String,AsyncCorutineDescriptor>
}
#[derive(Debug)]
pub struct AsyncCorutineDescriptor{
    pub name:String,
    pub tokens: Vec<token::Token>,
    pub instructions:Vec<Stantement>,
}

impl SwarmDescriptor {
    pub fn new (name:String,parameters:Vec<String>,pipes:Vec<String>,internal_pipes:Vec<String>,instructions: Vec<token::Token>) -> Self {
        let mut instruction_slicer = (0,instructions.len());
        if instructions[0] == token::Token::LBRACE && instructions[instructions.len() -1 ] == token::Token::RBRACE{
            instruction_slicer = (1,instructions.len() -1 );
        } 
        let to_save = instructions[instruction_slicer.0..instruction_slicer.1].to_vec();
        let swarm = SwarmDescriptor {name,parameters,io_pipes:pipes,internal_pipes,instructions:to_save,corutines:HashMap::new()};
        return swarm;
    }
}

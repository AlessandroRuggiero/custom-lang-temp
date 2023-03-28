use crate::lexer::token;

#[derive(Debug)]
pub struct SwarmDescriptor {
    pub name:String,
    pub parameters: Vec<String>,
    pub io_pipes:Vec<String>,
    pub internal_pipes:Vec<String>,
    pub instructions: Vec<token::Token>
}

impl SwarmDescriptor {
    pub fn new (name:String,parameters:Vec<String>,pipes:Vec<String>,internal_pipes:Vec<String>,instructions: Vec<token::Token>) -> Self {
        let swarm = SwarmDescriptor {name,parameters,io_pipes:pipes,internal_pipes,instructions};
        return swarm;
    }
}
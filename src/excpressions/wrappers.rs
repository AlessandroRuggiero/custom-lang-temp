use std::collections::HashMap;
use crate::lexer::token;
use crossbeam_channel::{bounded, Sender,Receiver};
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
#[derive(Debug,Clone)]
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
#[derive(Debug)]
pub struct Swarm {
    pub swarm:SwarmDescriptor,
    pub pipes: HashMap<String,Pipe>,
}

impl Swarm {
    pub fn new (sd:SwarmDescriptor) -> Self {
        Self { swarm: sd, pipes:HashMap::new()}
    }
}

#[derive(Debug)]
pub struct AsyncCorutine {
    pub corutine:AsyncCorutineDescriptor,
    pub variables: HashMap<String,()>,
    pub i_counter:usize
}

impl AsyncCorutine {
    pub fn new (sd:AsyncCorutineDescriptor) -> Self {
        Self { corutine: sd, i_counter:0,variables:HashMap::new()}
    }
}

#[derive(Debug,Clone)]
pub struct Pipe {
    pub sender: Option<Sender<String>>,
    pub receiver: Option<Receiver<String>>
}

impl Pipe {
    pub fn new(sender: Option<Sender<String>>, receiver: Option<Receiver<String>>) -> Self { Self { sender, receiver } }
    pub fn send (&self,s:String) -> Result<(),&str>{
        match &self.sender {
            Some (sender) => {sender.send(s).unwrap();Ok (())},
            None => Err("Impossible to send data in this pipe")
        }
    }
}

impl Default for Pipe {
    fn default() -> Self { 
        let (sender,receiver) = bounded(0);
        Self {sender: Some(sender), receiver:Some(receiver) }
    }


}
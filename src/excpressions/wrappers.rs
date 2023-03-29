use std::{collections::HashMap, ops::{Add, Sub}};
use crate::lexer::token;
use crossbeam_channel::{bounded, Sender,Receiver};
use super::expressions::{Stantement, Variable};

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
    pub variables: HashMap<String,Variable>,
    pub i_counter:usize
}

impl AsyncCorutine {
    pub fn new (sd:AsyncCorutineDescriptor) -> Self {
        Self { corutine: sd, i_counter:0,variables:HashMap::new()}
    }
}

#[derive(Debug,Clone)]
pub struct Pipe {
    pub sender: Option<Sender<Message>>,
    pub receiver: Option<Receiver<Message>>
}
#[derive(Debug)]
pub enum Message {
    MSG (Variable),
    CLOSE
}

impl Pipe {
    pub fn new(sender: Option<Sender<Message>>, receiver: Option<Receiver<Message>>) -> Self { Self { sender, receiver } }
    pub fn send (&self,s:Message) -> Result<(),&str>{
        match &self.sender {
            Some (sender) => {
                let res = sender.send(s);
                return match res {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        println!("Cant send data in channel: {}",e.to_string());
                        Err("Impossible to send data in channel")
                    },
                };}
            None => Err("Impossible to send data in this pipe")
        }
    }
    pub fn receive (&self) -> Result<Message, String> {
        match &self.receiver {
            Some(r) => {
                let message = r.recv();
                match message {
                    Ok(m) => Ok(m),
                    Err(e) => Err(format!("Error in channel rcv: {}",e.to_string())),
                }
            },
            None =>  Err("Impossible to recive data from this pipe".to_owned())
        }
    }
}

impl Default for Pipe {
    fn default() -> Self { 
        let (sender,receiver) = bounded(0);
        Self {sender: Some(sender), receiver:Some(receiver) }
    }


}
impl Add for Variable {
    type Output = Result<Variable,String>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (Variable::STRING(s), Variable::STRING(s2)) => Ok(Variable::STRING(s + &s2)),
            (Variable::STRING(_), Variable::INT(_)) => todo!(),
            (Variable::STRING(_), Variable::FLOAT(_)) => todo!(),
            (Variable::INT(_), Variable::STRING(_)) => todo!(),
            (Variable::INT(i1), Variable::INT(i2)) => Ok (Variable::INT(i1+i2)),
            (Variable::INT(n1), Variable::FLOAT(n2)) =>Ok (Variable::FLOAT((n1 as f64) + n2)),
            (Variable::FLOAT(_), Variable::STRING(_)) => todo!(),
            (Variable::FLOAT(n1), Variable::INT(n2)) => Ok (Variable::FLOAT(n1+(n2 as f64))),
            (Variable::FLOAT(n1), Variable::FLOAT(n2)) => Ok (Variable::FLOAT(n1+n2))
        }
    }
}

impl Sub for Variable {
    type Output = Result<Variable,String>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (Variable::STRING(_), Variable::STRING(_)) => todo!(),
            (Variable::STRING(_), Variable::INT(_)) => todo!(),
            (Variable::STRING(_), Variable::FLOAT(_)) => todo!(),
            (Variable::INT(_), Variable::STRING(_)) => todo!(),
            (Variable::INT(n1), Variable::INT(n2)) => Ok (Variable::INT(n1-n2)),
            (Variable::INT(n1), Variable::FLOAT(n2)) => Ok (Variable::FLOAT((n1 as f64) - n2)),
            (Variable::FLOAT(_), Variable::STRING(_)) => todo!(),
            (Variable::FLOAT(n1), Variable::INT(n2)) => Ok (Variable::FLOAT(n1 - (n2 as f64))),
            (Variable::FLOAT(n1), Variable::FLOAT(n2)) => Ok (Variable::FLOAT(n1-n2)),
        }
    }
}
use crate::{lexer::{Lexer, token}, excpressions::wrappers::SwarmDescriptor};

use super::{wrappers::AsyncCorutineDescriptor, expressions::Stantement,expressions::Expression};

pub fn find<'a, T: PartialEq> (v:&'a Vec<T>,el:T,err_str:&'static str) -> Result<usize,&'static str> {
    let res = v.iter().position(|e| *e == el);
    match res {
        None => Err(err_str),
        Some(e) => Ok(e)
    }
}

pub fn get_ident_name (i: &token::Token) -> Result<String,&'static str> {
    match i{
        token::Token::IDENT(v) => Ok(v.into_iter().collect()),
        _ => Err("swarm entity name is invalid")
    }
}

fn check_balanced_brakets (v:&[token::Token]) -> bool {
    let mut open = 0;
    let mut end = false;
    for t in v{
        //println!("{:?}",t);
        if end {
            println!("the end is near {:?}",t);
            return false;
        }
        match t {
            token::Token::LBRACE => {
                open+=1;
            },
            token::Token::RBRACE => {
                open-=1;
                if open < 0 {
                    return false;
                }
                if open == 0{
                    end = true;
                }
            },
            _ => {}

        };
    }
    return open == 0;
}


fn find_brace_end (instructions:&Vec<token::Token>) -> Result<usize, &'static str>{
        let mut open = 0;
        for (i,t) in instructions.iter().enumerate(){
            match t {
                token::Token::LBRACE => open+=1,
                token::Token::RBRACE => {
                    open-=1;
                    if open == 0 {
                        return Ok(i);
                    }
                },
                _ => {}
            }
        }
        Err("No end brace found")
}

pub fn parse_swarm (l:&mut Lexer) -> Result<SwarmDescriptor,&str>{
    l.read_char();
    let tokens:Vec<_>= l.into_iter().filter(|e| *e != token::Token::IGNORE).collect();
    let swarm_index  = find(&tokens,token::Token::SWARM,"No swarm start")?;
    let args_index  = find(&tokens,token::Token::LPAREN,"No args start")?;
    let args_end_index  = find(&tokens,token::Token::RPAREN,"No args end")?;
    let pipes_index  = find(&tokens,token::Token::LT,"No channels start")?;
    let pipes_end_index  = find(&tokens,token::Token::GT,"No channels end")?;
    let internal_pipes_index  = find(&tokens,token::Token::LBRAKET,"No channels start")?;
    let internal_pipes_end_index  = find(&tokens,token::Token::RBRAKET,"No channels end")?;
    if args_index - swarm_index != 2 || args_end_index <= args_index || pipes_index >= pipes_end_index || internal_pipes_index >= internal_pipes_end_index{
        return Err("Malformed swarm");
    }
    let swarm_name = get_ident_name(&tokens[swarm_index + 1])?;
    let parameters: Result<Vec<String>,&'static str>= (&tokens[args_index + 1..args_end_index]).iter().filter(|e| **e != token::Token::COMMA).map(get_ident_name).collect();
    let parameters = parameters?;
    let pipes: Result<Vec<String>,&'static str>= (&tokens[pipes_index + 1..pipes_end_index]).iter().filter(|e| **e != token::Token::COMMA).map(get_ident_name).collect();
    let pipes = pipes?;
    let internal_pipes: Result<Vec<String>,&'static str>= (&tokens[internal_pipes_index + 1..internal_pipes_end_index]).iter().filter(|e| **e != token::Token::COMMA).map(get_ident_name).collect();
    let internal_pipes = internal_pipes?;
    let body = &tokens[internal_pipes_end_index+1..];
    let balanced = check_balanced_brakets(body);
    if !balanced {
        return Err("Body brackets are not balanced")
    }
    let swarm = SwarmDescriptor::new(swarm_name,parameters,pipes,internal_pipes,body.to_vec());
    //println!("{:?}",swarm);
    
    return Ok(swarm);
}

impl SwarmDescriptor {
    pub fn parse_functions (& mut self) -> Result<(), String>{
        let mut end_brace_index = 0;
        let mut instructions = self.instructions.clone();
        while end_brace_index != instructions.len() {
            instructions = instructions[end_brace_index..].to_vec();
            //println!("{:?}",instructions);
            let curutine_index  = find(&instructions,token::Token::ASYNC,"No async function start")?;
            let corutine_name = get_ident_name(&instructions[curutine_index + 1])?;
            let intructions_start  = find(&instructions,token::Token::LBRACE,"Nobrace for functrion start")?;
            end_brace_index = find_brace_end(&instructions)?;
            self.corutines.insert(corutine_name.clone(), AsyncCorutineDescriptor{name:corutine_name,tokens:instructions[intructions_start+1..end_brace_index].to_vec(),instructions:Vec::new()});
            end_brace_index+=1;
        }
        //let keys:Vec<&String> = self.corutines.keys().collect();
        for (_,v) in self.corutines.iter_mut(){
            v.create_expressions()?;
            v.tokens.clear();
        }
        Ok(())
    }
}


impl AsyncCorutineDescriptor {
    pub fn get_expressions_instructions (&self) -> Vec<Vec<&token::Token>> {
        let mut expressions = Vec::new();
        let mut expression = Vec::new();
        for t in &self.tokens{
            if *t != token::Token::SEMICOLON {
                expression.push(t);
            }else{
                expressions.push(expression);
                expression = Vec::new();
            }
        }
        return expressions; 
    }
}

impl AsyncCorutineDescriptor {
    pub fn create_expressions (&mut self) -> Result<(),String> {
        let mut final_expression = Vec::new();
        let instructions = self.get_expressions_instructions();
        //println!("{:?}",instructions);
        for instruction in instructions{
            let mut stantement = None;
            if instruction[0] == &token::Token::IF {
                
            }
            else if instruction.contains(&&token::Token::ASSIGN) {
                let equals = find(&instruction, &&token::Token::ASSIGN, "Impossible to find = in assign expression")?;
                if equals != 1 {
                    return Err(format!("too many things on the left of the =, found at index: {}",equals));
                }
                stantement = Some(Stantement::ASSIGN(get_ident_name(instruction[0])?, Expression::new(instruction[2..].to_vec())));
                
            } else if instruction.contains(&&token::Token::PUT) {
                let equals = find(&instruction, &&token::Token::PUT, "Impossible to find <- in expression")?;
                if equals != 1 {
                    return Err(format!("too many things on the left of the <-, found at index: {}",equals));
                }
                stantement = Some(Stantement::PUT(get_ident_name(instruction[0])?, Expression::new(instruction[2..].to_vec())));
            } else if instruction.contains(&&token::Token::GET) {
                if instruction.len() != 3 {
                    return Err(format!("Wrong number of operands in pipe listen"));
                }
                stantement = Some(Stantement::GET(get_ident_name(instruction[0])?,get_ident_name(instruction[2])?));
            } else if instruction.len() == 1 {
                // single value;
                match instruction[0] {
                    token::Token::IDENT(v) => {
                        let v_name = get_ident_name(instruction[0])?;
                        panic!("should not get here");
                    },
                    v => return Err(format!("Invalid literal  {:?}", v))
                }
            }
            if let Some(exp) = stantement {
                final_expression.push(exp);
            }
        }
        self.instructions = final_expression;
        Ok(())
    }
}
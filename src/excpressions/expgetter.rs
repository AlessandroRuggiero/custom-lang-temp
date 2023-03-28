use crate::{lexer::{Lexer, token}, excpressions::wrappers::Swarm};

fn find<'a, T: PartialEq> (v:&'a Vec<T>,el:T,err_str:&'static str) -> Result<usize,&'static str> {
    let res = v.iter().position(|e| *e == el);
    match res {
        None => Err(err_str),
        Some(e) => Ok(e)
    }
}

fn get_ident_name (i: &token::Token) -> Result<String,&'static str> {
    match i{
        token::Token::IDENT(v) => Ok(v.into_iter().collect()),
        _ => Err("swarm entity name is invalid")
    }
}

pub fn parse_swarm (l:&mut Lexer) -> Result<(),&str>{
    l.read_char();
    let tokens:Vec<_>= l.into_iter().collect();
    let swarm_index  = find(&tokens,token::Token::SWARM,"No swarm start")?;
    let args_index  = find(&tokens,token::Token::LPAREN,"No args start")?;
    let args_end_index  = find(&tokens,token::Token::RPAREN,"No args end")?;
    if args_index - swarm_index != 2 || args_end_index == args_index{
        return Err("Malformed swarm");
    }
    let swarm_name = get_ident_name(&tokens[swarm_index + 1])?;
    let parameters: Result<Vec<String>,&'static str>= (&tokens[args_index + 1..args_end_index]).iter().filter(|e| **e != token::Token::COMMA).map(get_ident_name).collect();
    let parameters = parameters?;

    let swarm = Swarm::new(swarm_name,parameters);
    println!("{:?}",swarm);
    
    return Ok(());
}
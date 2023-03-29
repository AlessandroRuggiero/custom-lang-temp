use crate::lexer::token;

#[derive(Debug,Clone)]
pub enum Stantement {
    ASSIGN(String,Expression),
    PUT(String,Expression),
    GET(String,String),
}


#[derive(Debug,Clone)]
pub struct  Expression {
    pub instruction: Vec<token::Token>
}

impl Expression {
    pub fn new(instruction: Vec<&token::Token>) -> Self { 
        let mut i= Vec::with_capacity(instruction.len());
        for t in instruction{
            i.push(t.clone())
        }
        Self { instruction :i} 
    }
}

#[derive(Debug,Clone)]
pub enum Variable {
    STRING(String),
    INT(i64),
    FLOAT(f64)
}

impl ToString for Variable {
    fn to_string(&self) -> String {
        match self {
            Variable::STRING(v) => v.clone(),
            Variable::INT(i) => i.to_string(),
            Variable::FLOAT(f) => f.to_string(),
        }
    }
}
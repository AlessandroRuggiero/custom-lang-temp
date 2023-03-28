use crate::lexer::token;

#[derive(Debug)]
pub enum Stantement {
    ASSIGN(String,Expression)
}


#[derive(Debug)]
pub enum Expression {
    ADD(Vec<token::Token>),
    TOKENEXPRESSION (token::Token)
}

extern crate phf;

#[derive(PartialEq)]
#[derive(Debug,Clone)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(Vec<char>),
    INT(Vec<char>),
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRAKET,
    RBRAKET,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    PUT,
    GET,
    SWARM,
    LOOP,
    ASYNC,
    DOUBLEQUOTES,
    DOT,
    STRINGSPACE,
    IGNORE
}

pub fn get_keyword_token(ident: &Vec<char>) -> Result<Token, String> {
    let identifier: String = ident.into_iter().collect();
    match &identifier[..] {
        "fn" => Ok(Token::FUNCTION),
        "let" => Ok(Token::LET),
        "true" => Ok(Token::TRUE),
        "false" => Ok(Token::FALSE),
        "if" => Ok(Token::IF),
        "else" => Ok(Token::ELSE),
        "return" => Ok(Token::RETURN),
        "swarm" => Ok(Token::SWARM),
        "loop" => Ok(Token::LOOP),
        "async" => Ok(Token::ASYNC),
        _ => Err(String::from("Not a keyword"))
    }
}
use std::fs;

mod lexer;
mod excpressions;

fn main() {
    let contents = fs::read_to_string("examples/text.txt").expect("Should have been able to read the file");
    let input = String::from(contents);
    let mut l = lexer::Lexer::new(input.chars().collect());
    l.read_char();
    loop {
        let token = l.next_token();
        if token == lexer::token::Token::EOF {
            break;
        } else if token == lexer::token::Token::SEMICOLON(';') {
            println!("\n");
        } else if token == lexer::token::Token::LBRACE('{') {
            println!("{}\n","{");
        }else {
            println!("{:?}", token);
        }
    }
    println!("{} {} {}", char::from(l.ch), l.position, l.read_position);
}

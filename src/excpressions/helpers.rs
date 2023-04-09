use crate::lexer::token::Token;

pub fn next_block (tokens:Vec<Token>,delimiter_open:&Token,delimiter_close:&Token) ->  Option<(Vec<Token>,usize,usize)> {
    let mut open = 0;
    let mut open_index = None;
    let mut close_index = None;
    for (i,t) in tokens.iter().enumerate() {
        if t == delimiter_open {
            if open == 0 {
                open_index = Some(i);
            }
            open +=1;
        } else if delimiter_close == t {
            open -=1;
            if open == 0 {
                close_index = Some(i);
                break;
            }
        }
    }
    let oi = open_index?;
    let ci = close_index?;
    return Some((tokens[oi..ci].to_vec(),oi,ci));
}
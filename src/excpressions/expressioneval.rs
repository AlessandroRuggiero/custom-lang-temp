use crate::{lexer::token, excpressions::expgetter::get_ident_name};

use super::expressions::{Expression, Variable};

impl Expression {
    pub fn exaluate (&self) -> Result<Variable,String> {
        if self.instruction[0] == token::Token::DOUBLEQUOTES && self.instruction[self.instruction.len() - 1] == token::Token::DOUBLEQUOTES {
            let maybe_string = self.instruction[1..self.instruction.len() - 1].to_vec();
            if maybe_string.contains(&token::Token::DOUBLEQUOTES) {
                return Err(format!("{:?} in an invalid string",maybe_string));
            }
            return Ok(Variable::STRING(get_ident_name(&maybe_string[0]).expect("not ident")));
        }
        Err("invalid expression".to_owned())
    }
}
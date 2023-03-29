use crate::{lexer::token, excpressions::expgetter::get_ident_name};

use super::{expressions::{Expression, Variable}, wrappers::AsyncCorutine};

impl Expression {
    pub fn exaluate (&self, f:&AsyncCorutine) -> Result<Variable,String> {
        if self.instruction[0] == token::Token::DOUBLEQUOTES && self.instruction[self.instruction.len() - 1] == token::Token::DOUBLEQUOTES {
            let maybe_string = self.instruction[1..self.instruction.len() - 1].to_vec();
            if maybe_string.contains(&token::Token::DOUBLEQUOTES) {
                return Err(format!("{:?} in an invalid string",maybe_string));
            }
            return Ok(Variable::STRING(get_ident_name(&maybe_string[0]).expect("not ident")));
        } else if self.instruction.len () == 1{
            return match &self.instruction[0] {
                token::Token::IDENT(_) => {
                    let v_name = get_ident_name(&self.instruction[0])?;
                    let variable = f.variables.get(&v_name).ok_or(format!("{} variable not found",v_name))?;
                    Ok(variable.clone())
                },
                t => Err(format!("Invalid literal: {:?}",t))
            };
        }
        Err("invalid expression".to_owned())
    }
}
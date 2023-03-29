use std::ops::Index;

use crate::{lexer::token, excpressions::expgetter::get_ident_name};

use super::{expressions::{Expression, Variable}, wrappers::AsyncCorutine};

impl Expression {
    pub fn evaluate (&self, f:&AsyncCorutine) -> Result<Variable,String> {
        if self.instruction[0] == token::Token::DOUBLEQUOTES && self.instruction[self.instruction.len() - 1] == token::Token::DOUBLEQUOTES {
            //println!("{:?}",self.instruction);
            let maybe_string = self.instruction[1..self.instruction.len() - 1].to_vec();
            if maybe_string.contains(&token::Token::DOUBLEQUOTES) {
                return Err(format!("{:?} in an invalid string",maybe_string));
            }
            let mut stringa:Vec<char> = Vec::new();
            for t in &maybe_string{
                match t {
                    token::Token::IDENT(i) => stringa.extend(i),
                    token::Token::STRINGSPACE => stringa.push (' '),
                    _ => return Err("Ivalid string literal".to_owned()),
                }
            }
            return Ok(Variable::STRING(stringa.iter().collect::<String>()));
        } else if self.instruction.len () == 1{
            return match &self.instruction[0] {
                token::Token::IDENT(_) => {
                    let v_name = get_ident_name(&self.instruction[0])?;
                    let variable = f.variables.get(&v_name).ok_or(format!("{} variable not found",v_name))?;
                    Ok(variable.clone())
                },
                token::Token::INT(i) => {
                    let v:String = i.into_iter().collect();
                    let parsed = v.parse::<i64>();
                    match parsed {
                        Ok(v) => Ok(Variable::INT(v)),
                        Err(e) => Err(format!("error prasing number: {:?}",e)),
                    }
                },
                t => Err(format!("Invalid literal: {:?}",t))
            };
        
        } else if self.instruction.contains(&token::Token::PLUS)  {
            let splitter = self.instruction.iter().position(|e| e == &token::Token::PLUS).unwrap();
            let left:Vec<&token::Token> = self.instruction[0..splitter].iter().map(|e| e ).collect(); 
            let left = Expression::new(left);
            let rigth:Vec<&token::Token> = self.instruction[splitter+1..].iter().map(|e| e ).collect(); 
            let rigth = Expression::new(rigth);
            let vl = left.evaluate(f)?;
            let vr = rigth.evaluate(f)?;
            return vl + vr;
            // let rigth = Expression::new();
        }else if self.instruction.len() == 3 {
            //parsing a float
            let triplet = (&self.instruction[0],&self.instruction[1],&self.instruction[2]);
            match triplet {
                (token::Token::INT(a),token::Token::DOT,token::Token::INT(b)) => {
                    let a:String = a.into_iter().collect();
                    let b:String = b.into_iter().collect();
                    let val = format!("{}.{}",a,b);
                    let f = val.parse::<f64>();
                    return match f {
                        Ok(fl) => Ok(Variable::FLOAT(fl)),
                        Err(e) => Err(format!("Error parsing float value {} -- {}",e,val)),
                    };
                    
                },
                (_,_,_) => panic!("invalid froat notation")
            }
        }
        Err("invalid expression".to_owned())
    }
}
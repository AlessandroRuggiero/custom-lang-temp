use std::fs;

mod lexer;
mod excpressions;

fn main() {
    let contents = fs::read_to_string("examples/text.txt").expect("Should have been able to read the file");
    let input = String::from(contents);
    let mut l = lexer::Lexer::new(input.chars().collect());
    let res = excpressions::expgetter::parse_swarm(&mut l);
    match res {
        Ok(mut swarm)=> {
            swarm.parse_functions().unwrap();
            //swarm.corutines.get("main").unwrap().create_expressions();
            println!("Program processed correctly {:?}",swarm.corutines.get("main").unwrap());
            
        },
        Err(e) => println!("Error parsing expressions: {}",e)
    }
}

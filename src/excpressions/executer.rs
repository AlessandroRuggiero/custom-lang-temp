use std::{thread::{self, JoinHandle}, collections::HashMap, io};
use crossbeam_channel::bounded;

use super::{wrappers::{Swarm, AsyncCorutine, Pipe, Message}, expressions::{Stantement, Variable}};

impl Swarm {
    fn swarm_setup (&mut self) -> Vec<JoinHandle<()>>{
        // std out 
        let (i,o) = bounded::<Message>(0);
        let std_out = Pipe::new(Some(i),None);
        self.pipes.insert(self.swarm.io_pipes[1].clone(), std_out);
        let handle = thread::spawn(move || {
            loop {
                let data = o.recv().expect("Failed to recive");
                match data {
                    Message::MSG(d) => println!("{}",d.to_string()),
                    Message::CLOSE => {
                        //println!("Closing pipe");
                        break;
                    },
                };
            }
        });

        //std in
        let (i,o) = bounded::<Message>(0);
        let std_in = Pipe::new(None,Some(o));
        self.pipes.insert(self.swarm.io_pipes[0].clone(), std_in);
        let _ = thread::spawn(move || {
            loop {
                let stdin = io::stdin();
                let mut user_input = String::new();
                let res = stdin.read_line(&mut user_input);
                user_input = user_input[..(user_input.len()-1)].to_string();  // removing new line
                match res {
                    Ok(_) => {
                        let r = i.send(Message::MSG(Variable::STRING(user_input)));
                        match r {
                            Ok(_) => {},
                            Err(e) => println!("Cant send data in srd in {:?}",e),
                        }
                    },
                    Err(e) => println!("Cant read from standard in {:?}",e),
                }
            }
        });

        // setup internal pipes
        for p in &self.swarm.internal_pipes {
            let (i,o) = bounded::<Message>(0);
            let pipe = Pipe::new(Some(i),Some(o));
            self.pipes.insert(p.clone(), pipe);
        }
        return vec![handle];
    }
    pub fn execute_corutines (&mut self) {
        let io_handles = self.swarm_setup();
        let keys:Vec<&String> = self.swarm.corutines.keys().clone().collect();
        let keys = keys.clone();
        let mut handles = Vec::with_capacity(self.swarm.corutines.len());
        for corutine in keys {
            let cr = self.swarm.corutines.get(corutine).expect("invalid key got from map").clone();
            let mut cr = AsyncCorutine::new(cr);
            let pipes = self.pipes.clone();
            //println!("started: {}",cr.corutine.name);
            handles.push(thread::spawn(move || {
                cr.execute(pipes);
            }));
        }
        for c in handles{
            c.join().unwrap();
        }
        self.pipes.get(&self.swarm.io_pipes[1]).expect("no std out").send(Message::CLOSE).expect("Cant close stdout");
        for t in io_handles{
            //println!("Waiting for io threads to close");
            t.join().unwrap();
            //println!("Io threads closed");
        }
    }
}

impl AsyncCorutine {
    pub fn execute (&mut self, pipes:HashMap<String,Pipe>) {
        //println!("Executing: {:?}",self.corutine.instructions.clone());
        while self.i_counter != self.corutine.instructions.len(){
            let instruction = &self.corutine.instructions[self.i_counter];
            match instruction {
                Stantement::ASSIGN(var, exp) => {
                    let v = exp.evaluate(self).expect("Error evaluating rigth of =");
                    self.variables.insert(var.clone(), v);
                },
                Stantement::PUT(c, v) => {
                    let var = v.evaluate(&self).expect("expression evaluation failed");
                    //println!("{:?}",pipes);
                    let pipe = pipes.get(c).expect("Impossible to find pipe where to put the data");
                    pipe.send(Message::MSG(var)).expect("Cant send message varible in pipe");
                },
                Stantement::GET(c,v ) => {
                    let pipe = pipes.get(c).expect("impossible to find pipe");
                    let msg = pipe.receive();
                    match msg {
                        Ok(m) => {
                            match m {
                                Message::MSG(var) => {self.variables.insert(v.to_string(),var );
                                },
                                Message::CLOSE => break,
                            }
                        },
                        Err(e) => panic!("Impossible to find pipe"),
                    }
                },
                _ => panic!("invalid stantement")
            }
            self.i_counter += 1;
    }
}
}
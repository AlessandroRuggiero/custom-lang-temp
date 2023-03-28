use std::{thread, collections::HashMap, time::Duration};
use crossbeam_channel::bounded;

use super::{wrappers::{Swarm, AsyncCorutine, Pipe}, expressions::Stantement};

impl Swarm {
    fn swarm_setup (&mut self) {
        let (i,o) = bounded::<String>(0);
        let std_out = Pipe::new(Some(i),None);
        self.pipes.insert("out".to_owned(), std_out);
        thread::spawn(move || {
            loop {
                let data = o.recv().unwrap();
                println!("{}",data);
            }
        });
    }
    pub fn execute_corutines (&mut self) {
        self.swarm_setup();
        let keys:Vec<&String> = self.swarm.corutines.keys().clone().collect();
        let keys = keys.clone();
        let mut handles = Vec::with_capacity(self.swarm.corutines.len());
        for corutine in keys {
            let cr = self.swarm.corutines.get(corutine).expect("invalid key got from map").clone();
            let mut cr = AsyncCorutine::new(cr);
            let pipes = self.pipes.clone();
            handles.push(thread::spawn(move || {
                cr.execute(pipes);
            }));
        }
        for c in handles{
            c.join().unwrap();
        }
    }
}

impl AsyncCorutine {
    pub fn execute (&mut self, pipes:HashMap<String,Pipe>) {
        let instruction = &self.corutine.instructions[self.i_counter];
        match instruction {
            Stantement::PUT(c, v) => {
                let var = v.exaluate().expect("expression evaluation failed");
                let pipe = pipes.get(c).unwrap();
                pipe.send(format!("{:?}",var.to_string())).unwrap();
            },
            _ => panic!("invalid stantement")
        }
    }
}
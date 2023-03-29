use std::{thread::{self, JoinHandle}, collections::HashMap};
use crossbeam_channel::bounded;

use super::{wrappers::{Swarm, AsyncCorutine, Pipe, Message}, expressions::{Stantement}};

impl Swarm {
    fn swarm_setup (&mut self) -> Vec<JoinHandle<()>>{
        let (i,o) = bounded::<Message>(0);
        let std_out = Pipe::new(Some(i),None);
        self.pipes.insert("out".to_owned(), std_out);
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
            //println!("Channel closing");
        });
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
            handles.push(thread::spawn(move || {
                cr.execute(pipes);
            }));
        }
        for c in handles{
            c.join().unwrap();
        }
        self.pipes.get("out").expect("no std out").send(Message::CLOSE).expect("Cant close stdout");
        for t in io_handles{
            //println!("Waiting for io threads to close");
            t.join().unwrap();
            //println!("Io threads closed");
        }
    }
}

impl AsyncCorutine {
    pub fn execute (&mut self, pipes:HashMap<String,Pipe>) {
        while self.i_counter != self.corutine.instructions.len(){
            let instruction = &self.corutine.instructions[self.i_counter];
            match instruction {
                Stantement::PUT(c, v) => {
                    let var = v.exaluate().expect("expression evaluation failed");
                    let pipe = pipes.get(c).unwrap();
                    pipe.send(Message::MSG(var)).expect("Cant send message varible in pipe");
                },
                _ => panic!("invalid stantement")
            }
            self.i_counter += 1;
    }
}
}
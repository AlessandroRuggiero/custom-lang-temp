#[derive(Debug)]
pub struct Swarm {
    pub name:String,
    pub parameters: Vec<String>
}

impl Swarm {
    pub fn new (name:String,parameters:Vec<String>) -> Self {
        let swarm = Swarm {name,parameters};
        return swarm;
    }
}
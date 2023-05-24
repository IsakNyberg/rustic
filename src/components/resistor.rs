use super::Identifer;

/*
* This struct represents a resistor in a circuit
* It has an identifer, resistance, and two nodes.
*/
#[derive(Debug, Clone)]
pub struct Resistor {
    pub identifer: Identifer,
    pub resistance: f64,
    pub node1: usize,
    pub node2: usize,
}

impl Resistor {
    pub fn new(identifer: Identifer, resistance: f64, node1: usize, node2: usize) -> Self {
        let res = Self {
            identifer,
            resistance,
            node1,
            node2,
        };
        res
    }

    pub fn set_node1(&mut self, node1: usize) {
        self.node1 = node1;
    }

    pub fn set_node2(&mut self, node2: usize) {
        self.node2 = node2;
    }

    pub fn get_id(&self) -> usize {
        self.identifer.id
    }
}

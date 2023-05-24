use super::Identifer;
/*
* This struct is a ground node in a circuit it has a potential and has a single connection to a node.
*/
#[derive(Debug, Clone)]
pub struct Ground {
    pub identifer: Identifer,
    pub node: usize,
}

impl Ground {
    pub fn new(identifer: Identifer, node: usize) -> Ground {
        Ground { identifer, node }
    }

    pub fn set_node(&mut self, node: usize) {
        self.node = node;
    }

    pub fn get_id(&self) -> usize {
        self.identifer.id
    }
}

use super::{Connection, ConnectionType, Identifer};
/*
* This struct represents a node in a circuit. NOTE: a not is NOT a component.
* It contains the nodes id, potential, connections to and from the node, it can be locked or unlocked.
*/

#[derive(Clone)]
pub struct Node {
    pub identifer: Identifer,
    pub potential: f64,
    pub locked: bool,
    pub connections: Vec<Connection>,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node ({}) locked: {} potential: {} connections: {:?}",
            self.identifer.id, self.locked, self.potential, self.connections
        )
    }
}

impl Node {
    pub fn new(identifer: Identifer) -> Self {
        Self {
            identifer,
            potential: 0.0,
            locked: false,
            connections: Vec::new(),
        }
    }

    pub fn get_id(&self) -> usize {
        self.identifer.id
    }

    pub fn get_name(&self) -> String {
        self.identifer.name.clone()
    }

    pub fn set_potential(&mut self, potential: f64) {
        self.potential = potential;
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn add_connection(&mut self, comp_id: usize, con_type: ConnectionType) {
        let connection = Connection::Connected(comp_id, con_type);
        self.connections.push(connection);
    }
}

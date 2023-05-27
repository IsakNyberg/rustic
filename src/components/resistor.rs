use super::{Connection, Connection::*, ConnectionType::*, Identifer};

/*
* This struct represents a resistor in a circuit
* It has an identifer, resistance, and two nodes.
*/
#[derive(Clone)]
pub struct Resistor {
    pub identifer: Identifer,
    pub resistance: f64,
    pub node1: Connection,
    pub node2: Connection,
}

impl std::fmt::Debug for Resistor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "R ({}) R:{} Node1: {:?} Node2: {:?}",
            self.identifer.id, self.resistance, self.node1, self.node2
        )
    }
}

impl Resistor {
    pub fn new(identifer: Identifer, resistance: f64) -> Self {
        let res = Self {
            identifer,
            resistance,
            node1: Disconnected(Annode),
            node2: Disconnected(Cathode),
        };
        res
    }

    pub fn connect(&mut self, connection: &Connection) {
        match *connection {
            Connected(nodeid, connection_type) => match connection_type {
                Annode => self.node1 = Connected(nodeid, Annode),
                Cathode => self.node2 = Connected(nodeid, Cathode),
                _ => panic!("Resistor can only be connected to an Annode or Cathode"),
            },
            Disconnected(con_type) => match con_type {
                Annode => self.node1 = Disconnected(Annode),
                Cathode => self.node2 = Disconnected(Cathode),
                _ => panic!("Resistor can only be disconnected to an Annode or Cathode"),
            },
        };
    }

    pub fn get_id(&self) -> usize {
        self.identifer.id
    }
}

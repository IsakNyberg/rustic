use super::{
    Connection,
    Connection::*,
    ConnectionType::{self, *},
    Identifer,
};

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
        Self {
            identifer,
            resistance,
            node1: Disconnected(Anode),
            node2: Disconnected(Cathode),
        }
    }

    pub fn connect(&mut self, connection: &Connection) {
        match *connection {
            Connected(_, con_type) => match con_type {
                Anode => self.node1 = *connection,
                Cathode => self.node2 = *connection,
                _ => panic!("Resistor can only be connected to an Anode or Cathode"),
            },
            Disconnected(con_type) => match con_type {
                Anode => self.node1 = *connection,
                Cathode => self.node2 = *connection,
                _ => panic!("Resistor can only be disconnected to an Anode or Cathode"),
            },
        };
    }

    pub fn get_connection(&self, connection_type: ConnectionType) -> Connection {
        match connection_type {
            Anode => self.node1.clone(),
            Cathode => self.node2.clone(),
            _ => panic!("Resistor only has a Anode or Cathode"),
        }
    }

    pub fn get_id(&self) -> usize {
        self.identifer.id
    }
}

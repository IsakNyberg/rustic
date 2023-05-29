use super::{
    Connection,
    Connection::*,
    ConnectionType::{self, *},
    Identifer,
};
/*
* This struct is a ground node in a circuit it has a potential and has a single NodeConnection.
*/
#[derive(Clone)]
pub struct Ground {
    pub identifer: Identifer,
    pub node: Connection,
}

impl std::fmt::Debug for Ground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GND ({}) Node: {:?}", self.identifer.id, self.node)
    }
}

impl Ground {
    pub fn new(identifer: Identifer) -> Ground {
        Ground {
            identifer,
            node: Disconnected(GroundConnection),
        }
    }

    pub fn connect(&mut self, connection: &Connection) {
        match *connection {
            Connected(nodeid, connection_type) => match connection_type {
                GroundConnection => self.node = Connected(nodeid, GroundConnection),
                _ => panic!("Ground can only be connected to a GroundConnection"),
            },
            Disconnected(con_type) => match con_type {
                GroundConnection => self.node = Disconnected(GroundConnection),
                _ => panic!("Ground can only be disconnected to a GroundConnection"),
            },
        };
    }

    pub fn get_connection(&self, connection_type: ConnectionType) -> Connection {
        match connection_type {
            GroundConnection => self.node.clone(),
            _ => panic!("Ground only has a GroundConnection"),
        }
    }

    pub fn get_id(&self) -> usize {
        self.identifer.id
    }
}

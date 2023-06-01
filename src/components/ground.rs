use super::{
    ComponentTrait, Connection,
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
}

const PANIC_TEXT: &'static str = "Ground can only has connection type GroundConnection";

impl ComponentTrait for Ground {
    fn get_id(&self) -> usize {
        self.identifer.id
    }

    fn get_name(&self) -> String {
        self.identifer.name.clone()
    }

    fn connect(&mut self, node_id: usize, connection_type: ConnectionType) {
        match connection_type {
            GroundConnection => self.node = Connected(node_id, GroundConnection),
            _ => unreachable!("{PANIC_TEXT}"),
        };
    }

    fn disconnect(&mut self, connection_type: ConnectionType) {
        match connection_type {
            GroundConnection => self.node = Disconnected(GroundConnection),
            _ => unreachable!("{PANIC_TEXT}"),
        };
    }

    fn get_connection(&self, connection_type: ConnectionType) -> Connection {
        match connection_type {
            GroundConnection => self.node.clone(),
            _ => unreachable!("{PANIC_TEXT}"),
        }
    }

    fn current_representative(&self, index: usize, conn_type: ConnectionType, eq: &mut [f64]) {
        // When a node asks what current the connection provies to the node we
        // return depending on the connection type
        match conn_type {
            GroundConnection => eq[index] = -1.0, // current flows out of the node
            _ => unreachable!("{PANIC_TEXT}"),
        }
    }

    fn num_eq(&self) -> usize {
        1
    }

    fn equation(&self, _: usize, equation: &mut [f64], eq_id: usize) -> f64 {
        // V = 0
        assert!(eq_id < self.num_eq());
        let v = self.node.get_id();
        equation[v] = 1.0;
        0.0
    }
}

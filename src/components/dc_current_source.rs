use super::{
    ComponentTrait, Connection,
    Connection::*,
    ConnectionType::{self, *},
    Identifer,
};

/*
* This struct represents a DC current source in a circuit.
* It has an id, current, max_current, and anode and a cathode.
*/

#[derive(Clone)]
pub struct DCCurrentSource {
    pub identifer: Identifer,
    pub current: f64,
    pub anode: Connection,
    pub cathode: Connection,
}

impl DCCurrentSource {
    pub fn new(identifer: Identifer, current: f64) -> DCCurrentSource {
        DCCurrentSource {
            identifer,
            current,
            anode: Disconnected(Anode),
            cathode: Disconnected(Cathode),
        }
    }
}

impl std::fmt::Debug for DCCurrentSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DCCS ({}) I:{}A Anode: {:?} Cathode: {:?}",
            self.identifer.id, self.current, self.anode, self.cathode
        )
    }
}

const PANIC_TEXT: &'static str = "DC CS can only has connection type Anode or Cathode";

impl ComponentTrait for DCCurrentSource {
    fn get_id(&self) -> usize {
        self.identifer.id
    }
    fn get_name(&self) -> String {
        self.identifer.name.clone()
    }

    fn connect(&mut self, node_id: usize, connection_type: ConnectionType) {
        match connection_type {
            Anode => self.anode = Connected(node_id, Anode),
            Cathode => self.cathode = Connected(node_id, Cathode),
            _ => unreachable!("{PANIC_TEXT}"),
        };
    }

    fn disconnect(&mut self, connection_type: ConnectionType) {
        match connection_type {
            Anode => self.anode = Disconnected(Anode),
            Cathode => self.cathode = Disconnected(Cathode),
            _ => unreachable!("{PANIC_TEXT}"),
        }
    }

    fn get_connection(&self, connection_type: ConnectionType) -> Connection {
        match connection_type {
            Anode => self.anode.clone(),
            Cathode => self.cathode.clone(),
            _ => unreachable!("{PANIC_TEXT}"),
        }
    }

    fn current_representative(&self, index: usize, conn_type: ConnectionType, eq: &mut [f64]) {
        // When a node asks what current the connection provies to the node we
        // return depending on the connection type
        match conn_type {
            Anode => eq[index] = -1.0,  // current flows out of the node
            Cathode => eq[index] = 1.0, // current flows into the node
            _ => unreachable!("{PANIC_TEXT}"),
        }
    }

    fn num_eq(&self) -> usize {
        1
    }

    fn equation(&self, offset: usize, equation: &mut [f64], eq_id: usize) -> f64 {
        // When a node asks for the equation we return the current
        // equation[offset] = self.current;
        assert!(eq_id < self.num_eq());
        equation[offset] = 1.0;
        self.current
    }
}

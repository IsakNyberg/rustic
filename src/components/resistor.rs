use super::{
    ComponentTrait, Connection,
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
}

const PANIC_TEXT: &'static str = "Resistor can only has connection type Anode or Cathode";

impl ComponentTrait for Resistor {
    fn get_id(&self) -> usize {
        self.identifer.id
    }

    fn get_name(&self) -> String {
        self.identifer.name.clone()
    }

    fn connect(&mut self, node: usize, connection_type: ConnectionType) {
        match connection_type {
            Anode => self.node1 = Connected(node, Anode),
            Cathode => self.node2 = Connected(node, Cathode),
            _ => unreachable!("{PANIC_TEXT}"),
        };
    }

    fn disconnect(&mut self, connection_type: ConnectionType) {
        match connection_type {
            Anode => self.node1 = Disconnected(Anode),
            Cathode => self.node2 = Disconnected(Cathode),
            _ => unreachable!("{PANIC_TEXT}"),
        };
    }

    fn get_connection(&self, connection_type: ConnectionType) -> Connection {
        match connection_type {
            Anode => self.node1.clone(),
            Cathode => self.node2.clone(),
            _ => unreachable!("{PANIC_TEXT}"),
        }
    }

    fn current_representative(&self, index: usize, conn_type: ConnectionType, eq: &mut [f64]) {
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
        // V = IR
        // V = V1 - V2 = IR
        // V1 - V2 = IR
        // V1 - V2 - IR = 0
        assert!(eq_id < self.num_eq());
        let v1 = self.node1.get_id();
        let v2 = self.node2.get_id();
        let r = self.resistance;
        equation[v1] = 1.0;
        equation[v2] = -1.0;
        equation[offset] = -r;
        0.0
    }
}

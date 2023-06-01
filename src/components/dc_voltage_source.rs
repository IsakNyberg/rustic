use super::{
    ComponentTrait, Connection,
    Connection::*,
    ConnectionType::{self, *},
    Identifer,
};

/*
* This struct represents a DC voltage source in a circuit.
* It has an id, voltage, max_current, and anode and a cathode.
*/

#[derive(Clone)]
pub struct DCVoltageSource {
    pub identifer: Identifer,
    pub voltage: f64,
    pub anode: Connection,
    pub cathode: Connection,
}

impl std::fmt::Debug for DCVoltageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DCVS ({}) V:{}V Anode: {:?} Cathode: {:?}",
            self.identifer.id, self.voltage, self.anode, self.cathode
        )
    }
}

impl DCVoltageSource {
    pub fn new(identifer: Identifer, voltage: f64) -> DCVoltageSource {
        DCVoltageSource {
            identifer,
            voltage,
            anode: Disconnected(Anode),
            cathode: Disconnected(Cathode),
        }
    }
}

const PANIC_TEXT: &'static str = "DC VS can only has connection type Anode or Cathode";

impl ComponentTrait for DCVoltageSource {
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

    fn equation(&self, _: usize, equation: &mut [f64], eq_id: usize) -> f64 {
        // self.voltage + v1 = v2
        // self.voltage = -v1 + v2
        assert!(eq_id < self.num_eq());
        let v1 = self.anode.get_id();
        let v2 = self.cathode.get_id();
        equation[v1] = -1.0;
        equation[v2] = 1.0;
        self.voltage
    }
}

use super::{
    Connection,
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

    pub fn connect(&mut self, connection: &Connection) {
        match *connection {
            Connected(nodeid, connection_type) => match connection_type {
                Anode => self.anode = Connected(nodeid, Anode),
                Cathode => self.cathode = Connected(nodeid, Cathode),
                _ => panic!("DC VS can only be connected to an Anode or Cathode"),
            },
            Disconnected(con_type) => match con_type {
                Anode => self.anode = Disconnected(Anode),
                Cathode => self.cathode = Disconnected(Cathode),
                _ => panic!("DC VS can only be disconnected to an Anode or Cathode"),
            },
        };
    }

    pub fn get_connection(&self, connection_type: ConnectionType) -> Connection {
        match connection_type {
            Anode => self.anode.clone(),
            Cathode => self.cathode.clone(),
            _ => panic!("DC VS only has a Anode or Cathode"),
        }
    }

    pub fn get_id(&self) -> usize {
        self.identifer.id
    }
}

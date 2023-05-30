use super::{
    Connection,
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

impl std::fmt::Debug for DCCurrentSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DCCS ({}) I:{}A Anode: {:?} Cathode: {:?}",
            self.identifer.id, self.current, self.anode, self.cathode
        )
    }
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

    pub fn connect(&mut self, connection: &Connection) {
        match *connection {
            Connected(nodeid, connection_type) => match connection_type {
                Anode => self.anode = Connected(nodeid, Anode),
                Cathode => self.cathode = Connected(nodeid, Cathode),
                _ => panic!("DC CS can only be connected to an Anode or Cathode"),
            },
            Disconnected(con_type) => match con_type {
                Anode => self.anode = Disconnected(Anode),
                Cathode => self.cathode = Disconnected(Cathode),
                _ => panic!("DC CS can only be disconnected to an Anode or Cathode"),
            },
        };
    }

    pub fn get_connection(&self, connection_type: ConnectionType) -> Connection {
        match connection_type {
            Anode => self.anode.clone(),
            Cathode => self.cathode.clone(),
            _ => panic!("DC CS only has a Anode or Cathode"),
        }
    }

    pub fn get_id(&self) -> usize {
        self.identifer.id
    }
}

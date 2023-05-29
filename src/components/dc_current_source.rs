use super::{
    Connection,
    Connection::*,
    ConnectionType::{self, *},
    Identifer,
};

/*
* This struct represents a DC current source in a circuit.
* It has an id, current, max_current, and annode and a cathode.
*/

#[derive(Clone)]
pub struct DCCurrentSource {
    pub identifer: Identifer,
    pub current: f64,
    pub annode: Connection,
    pub cathode: Connection,
}

impl std::fmt::Debug for DCCurrentSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DCCS ({}) I:{}A Annode: {:?} Cathode: {:?}",
            self.identifer.id, self.current, self.annode, self.cathode
        )
    }
}

impl DCCurrentSource {
    pub fn new(identifer: Identifer, current: f64) -> DCCurrentSource {
        DCCurrentSource {
            identifer,
            current,
            annode: Disconnected(Annode),
            cathode: Disconnected(Cathode),
        }
    }

    pub fn connect(&mut self, connection: &Connection) {
        match *connection {
            Connected(nodeid, connection_type) => match connection_type {
                Annode => self.annode = Connected(nodeid, Annode),
                Cathode => self.cathode = Connected(nodeid, Cathode),
                _ => panic!("DC CS can only be connected to an Annode or Cathode"),
            },
            Disconnected(con_type) => match con_type {
                Annode => self.annode = Disconnected(Annode),
                Cathode => self.cathode = Disconnected(Cathode),
                _ => panic!("DC CS can only be disconnected to an Annode or Cathode"),
            },
        };
    }

    pub fn get_connection(&self, connection_type: ConnectionType) -> Connection {
        match connection_type {
            Annode => self.annode.clone(),
            Cathode => self.cathode.clone(),
            _ => panic!("DC CS only has a Annode or Cathode"),
        }
    }

    pub fn get_id(&self) -> usize {
        self.identifer.id
    }
}

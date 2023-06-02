mod dc_current_source;
mod dc_voltage_source;
mod ground;
mod node;
mod resistor;
mod switch_spdt;

pub use self::dc_current_source::DCCurrentSource;
pub use self::dc_voltage_source::DCVoltageSource;
pub use self::ground::Ground;
pub use self::node::Node;
pub use self::resistor::Resistor;
pub use self::switch_spdt::SwitchSPDT;
pub use self::Connection::*;

/*
* An id struct that has name, id, short_name, and long_name.
*/
#[derive(Debug, Clone)]
pub struct Identifer {
    pub name: String,
    pub id: usize,
    pub short_name: String,
    pub long_name: String,
}

impl Identifer {
    pub fn new(name: String, id: usize, short_name: String, long_name: String) -> Self {
        Self {
            name,
            id,
            short_name,
            long_name,
        }
    }

    pub fn from_id(id: usize) -> Self {
        Self {
            name: id.to_string(),
            id,
            short_name: id.to_string(),
            long_name: id.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Connection {
    Connected(usize, ConnectionType), // A connection to a node current flows in and out
    Disconnected(ConnectionType),     // A connection to nothing
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionType {
    Anode,
    Cathode,
    GroundConnection,
    Left,
    Middle,
    Right,
    //
    Input,
    Output,
    Pin,
    Pin0,
    Pin1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
    Pin6,
    Pin7,
    Pin8,
    Pin9,
}

impl Connection {
    pub fn get_id(&self) -> usize {
        match self {
            Connected(id, _) => *id,
            Disconnected(_) => panic!("Disconnected connection has no id"),
        }
    }

    pub fn get_connection_type(&self) -> ConnectionType {
        match self {
            Connected(_, connection_type) => connection_type.clone(),
            Disconnected(connection_type) => connection_type.clone(),
        }
    }

    pub fn make_disconnect(&mut self) -> Connection {
        match self {
            Connected(_, connection_type) => {
                let connection = Disconnected(connection_type.clone());
                *self = connection.clone();
                connection
            }
            Disconnected(_) => panic!("Connection already disconnected"),
        }
    }
}

pub trait ComponentTrait {
    fn get_id(&self) -> usize;
    fn get_name(&self) -> String;
    fn connect(&mut self, node: usize, connection_type: ConnectionType);
    fn disconnect(&mut self, connection_type: ConnectionType);
    fn get_connection(&self, connection_type: ConnectionType) -> Connection;
    fn current_representative(&self, index: usize, conn_type: ConnectionType, eq: &mut [f64]);
    fn num_eq(&self) -> usize;
    fn equation(&self, offset: usize, equation: &mut [f64], eq_id: usize) -> f64;
}

pub type Component = Box<dyn ComponentTrait>;

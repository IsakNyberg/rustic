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
pub use self::Component::*;
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
    Input1,
    Output1,
    Output2,
    // Gate,
    // Drain,
    // Source,

    // This is not meant to be used but it serves as a reminder to always have
    // a catch all for all match statements.
    // This should enum never appear anywhere else in the code
    UnimplementedConnectionType,
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

/*
* this is an enum that contain every possible component
* that can be in a circuit.
*/
#[derive(Debug, Clone)]
pub enum Component {
    ResistorComponent(Resistor),
    DCVoltageSourceComponent(DCVoltageSource),
    GroundComponent(Ground),
    DCCurrentSourceComponent(DCCurrentSource),
    SwitchSPDTComponent(SwitchSPDT),

    // This is not meant to be used but it serves as a reminder to always have
    // a catch all for all match statements.
    // This should enum never appear anywhere else in the code
    UnimplementedComponent,
}

// pub trait ComponentTrait {
//     fn get_id(&self) -> usize;
//     fn get_name(&self) -> String;
//     fn connect(&mut self, node: usize, connection_type: ConnectionType);
//     fn get_connection(&self, connection_type: ConnectionType) -> Connection;
// }

impl Component {
    pub const fn get_currents(&self) -> usize {
        // &'static [ConnectionType] {
        // currents
        // "rank" in the matrix required to calculate component
        // use ConnectionType::*;
        match self {
            ResistorComponent(_) => 1,        // &[Cathode],
            DCVoltageSourceComponent(_) => 1, // &[Cathode],
            GroundComponent(_) => 1,          // &[Cathode],
            DCCurrentSourceComponent(_) => 1, // &[Cathode],
            SwitchSPDTComponent(_) => 2,      // &[Output1, Output2],
            _ => 0,                           // &[],
        }
    }

    pub fn get_id(&self) -> usize {
        match self {
            ResistorComponent(resistor) => resistor.get_id(),
            DCVoltageSourceComponent(dc_vs) => dc_vs.get_id(),
            GroundComponent(ground) => ground.get_id(),
            DCCurrentSourceComponent(dc_cs) => dc_cs.get_id(),
            SwitchSPDTComponent(switch) => switch.get_id(),
            _ => panic!("get_id not implemented for {self:?}"),
        }
    }
    pub fn get_name(&self) -> String {
        match self {
            ResistorComponent(resistor) => resistor.identifer.name.clone(),
            DCVoltageSourceComponent(dc_vs) => dc_vs.identifer.name.clone(),
            GroundComponent(ground) => ground.identifer.name.clone(),
            DCCurrentSourceComponent(dc_cs) => dc_cs.identifer.name.clone(),
            SwitchSPDTComponent(switch) => switch.identifer.name.clone(),
            _ => panic!("get_name not implemented for {self:?}"),
        }
    }
    pub fn connect(&mut self, node: usize, connection_type: ConnectionType) {
        let connection = Connection::Connected(node, connection_type);
        match self {
            ResistorComponent(resistor) => resistor.connect(&connection),
            DCVoltageSourceComponent(dc_vs) => dc_vs.connect(&connection),
            GroundComponent(ground) => ground.connect(&connection),
            DCCurrentSourceComponent(dc_cs) => dc_cs.connect(&connection),
            SwitchSPDTComponent(switch) => switch.connect(&connection),
            _ => panic!("connect not implemented for {self:?}"),
        }
    }

    pub fn get_connection(&self, connection_type: ConnectionType) -> Connection {
        match self {
            ResistorComponent(resistor) => resistor.get_connection(connection_type),
            DCVoltageSourceComponent(dc_vs) => dc_vs.get_connection(connection_type),
            GroundComponent(ground) => ground.get_connection(connection_type),
            DCCurrentSourceComponent(dc_cs) => dc_cs.get_connection(connection_type),
            SwitchSPDTComponent(switch) => switch.get_connection(connection_type),
            _ => panic!("get_connection not implemented for {self:?}"),
        }
    }
}

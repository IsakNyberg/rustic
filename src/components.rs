mod dc_voltage_source;
mod ground;
mod node;
mod resistor;

pub use self::dc_voltage_source::DCVoltageSource;
pub use self::ground::Ground;
pub use self::node::Node;
pub use self::resistor::Resistor;

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

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_short_name(&mut self, short_name: String) {
        self.short_name = short_name;
    }

    pub fn set_long_name(&mut self, long_name: String) {
        self.long_name = long_name;
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
}

pub trait ComponentTrait {
    fn get_id(&self) -> usize;
    fn get_name(&self) -> String;
    fn get_input_node(&self) -> usize;
    fn get_output_node(&self) -> usize;
    fn is_input_node(&self, node: usize) -> bool {
        self.get_input_node() == node
    }
    fn is_output_node(&self, node: usize) -> bool {
        self.get_output_node() == node
    }
}

impl ComponentTrait for Component {
    fn get_id(&self) -> usize {
        match self {
            Component::ResistorComponent(resistor) => resistor.get_id(),
            Component::DCVoltageSourceComponent(dc_vs) => dc_vs.get_id(),
            Component::GroundComponent(ground) => ground.get_id(),
        }
    }
    fn get_name(&self) -> String {
        match self {
            Component::ResistorComponent(resistor) => resistor.identifer.name.clone(),
            Component::DCVoltageSourceComponent(dc_vs) => dc_vs.identifer.name.clone(),
            Component::GroundComponent(ground) => ground.identifer.name.clone(),
        }
    }
    fn get_input_node(&self) -> usize {
        // input == annode
        match self {
            Component::ResistorComponent(resistor) => resistor.node1,
            Component::DCVoltageSourceComponent(dc_vs) => dc_vs.annode,
            Component::GroundComponent(ground) => ground.node,
        }
    }
    fn get_output_node(&self) -> usize {
        // output == cathode
        match self {
            Component::ResistorComponent(resistor) => resistor.node2,
            Component::DCVoltageSourceComponent(dc_vs) => dc_vs.cathode,
            Component::GroundComponent(ground) => ground.node,
        }
    }
}

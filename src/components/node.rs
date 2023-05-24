use super::{Component, Identifer};
/*
* This struct represents a node in a circuit. NOTE: a not is NOT a component.
* It contains the nodes id, potential, connections to and from the node, it can be locked or unlocked.
*/

#[derive(Debug, Clone)]
pub struct Node {
    pub identifer: Identifer,
    pub potential: f64,
    pub locked: bool,
    pub connections: Vec<usize>,
}

impl Node {
    pub fn new(identifer: Identifer) -> Self {
        Self {
            identifer,
            potential: 0.0,
            locked: false,
            connections: Vec::new(),
        }
    }

    pub fn get_id(&self) -> usize {
        self.identifer.id
    }

    pub fn get_name(&self) -> String {
        self.identifer.name.clone()
    }

    pub fn set_potential(&mut self, potential: f64) {
        self.potential = potential;
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn add_connection(&mut self, component: Component) {
        // unwarap the component and push its id to the connections vector
        let component_id = match component {
            Component::ResistorComponent(resistor) => resistor.identifer.id,
            Component::DCVoltageSourceComponent(dc_vs) => dc_vs.identifer.id,
            Component::GroundComponent(ground) => ground.identifer.id,
        };
        self.connections.push(component_id);
    }
}

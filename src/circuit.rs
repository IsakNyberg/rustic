use crate::components;

use components::Component;
use components::Node;

/*
* An id struct that has name, id, components, and nodes.
*/
#[derive(Debug, Clone)]
pub struct Circuit {
    pub name: String,
    pub id: usize,
    pub components: Vec<Component>,
    pub nodes: Vec<Node>,
    pub locked: bool,
}

impl Circuit {
    pub fn new(name: String, id: usize) -> Self {
        Self {
            name,
            id,
            components: Vec::new(),
            nodes: Vec::new(),
            locked: false,
        }
    }

    pub fn from_components_nodes(
        name: String,
        id: usize,
        components: Vec<Component>,
        nodes: Vec<Node>,
    ) -> Self {
        Self {
            name,
            id,
            components,
            nodes,
            locked: false,
        }
    }

    pub fn get_potential(&self, node_id: usize) -> f64 {
        self.nodes[node_id].potential
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }
}

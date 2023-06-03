use std::collections::HashMap;

use crate::components::Connection::{Connected, Disconnected};
use crate::components::ConnectionType;
use crate::components::Identifer;
use crate::components::Node;
use crate::components::{Component, ComponentTrait};

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
    pub comp_to_cur_index_map: HashMap<usize, usize>,
    pub num_variables: usize,
}

impl Circuit {
    pub fn new(name: String, id: usize) -> Self {
        Self {
            name,
            id,
            components: Vec::new(),
            nodes: Vec::new(),
            locked: false,
            comp_to_cur_index_map: HashMap::new(),
            num_variables: 0,
        }
    }

    pub fn from_components(name: String, id: usize, components: Vec<Component>) -> Self {
        Self {
            name,
            id,
            components,
            nodes: Vec::new(),
            locked: false,
            comp_to_cur_index_map: HashMap::new(),
            num_variables: 0,
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
            comp_to_cur_index_map: HashMap::new(),
            num_variables: 0,
        }
    }

    pub fn connect_nodes(&mut self, args: Vec<(usize, usize, ConnectionType)>) {
        for (comp_id, node_id, con_type) in args {
            self.connect_node(comp_id, node_id, con_type);
        }
    }

    pub fn get_potential_index(&self, node_id: usize) -> usize {
        // In the futire this logic may be more complicated and thus this function exists
        node_id
    }

    pub fn get_current_index(&self, component: &Component) -> usize {
        self.comp_to_cur_index_map[&component.get_id()]
    }

    pub fn connect_components(
        &mut self,
        args: Vec<((usize, ConnectionType), (usize, ConnectionType))>,
    ) {
        for (conn1, conn2) in args {
            self.connect_component(conn1, conn2);
        }
    }

    pub fn connect_component(
        &mut self,
        conn1: (usize, ConnectionType),
        conn2: (usize, ConnectionType),
    ) {
        let (comp1_id, con_type1) = conn1;
        let (comp2_id, con_type2) = conn2;
        assert!(comp1_id < self.components.len());
        assert!(comp2_id < self.components.len());
        // see if either node is connected already
        let node1_id = match self.components[comp1_id].get_connection(con_type1) {
            Connected(id, _) => id,
            Disconnected(_) => self.nodes.len(),
        };
        let node2_id = match self.components[comp2_id].get_connection(con_type2) {
            Connected(id, _) => id,
            Disconnected(_) => self.nodes.len(),
        };
        // See if both nodes are connected already, this means both are connected to a different node
        if node1_id != node2_id && node1_id != self.nodes.len() && node2_id != self.nodes.len() {
            panic!("Both components are already connected to different nodes");
        }
        // the new node is the min number of both nodes, this means that if one of the nodes is not connected yet
        // the new node will be the id of the other node. If neither is connected a new node will be created to connect the two
        let node_id = std::cmp::min(node1_id, node2_id);

        self.connect_node(comp1_id, node_id, con_type1);
        self.connect_node(comp2_id, node_id, con_type2);
    }

    pub fn connect_node(&mut self, comp_id: usize, node_id: usize, con_type: ConnectionType) {
        assert!(comp_id < self.components.len());

        while node_id >= self.nodes.len() {
            let identifier = Identifer::from_id(self.nodes.len());
            let node = Node::new(identifier);
            self.nodes.push(node);
        }

        // select the node and connect it
        let node = &mut self.nodes[node_id];
        node.add_connection(comp_id, con_type);
        // select the component and connect it
        let component = &mut self.components[comp_id];
        component.connect(node_id, con_type)
    }

    pub fn currents_at_node_eq(&self, node_id: usize, eq: &mut [f64]) {
        let node = &self.nodes[node_id];

        for connection in node.connections.iter() {
            // everything whose input is the cathode is added to the current
            let component_id = connection.get_id();
            let conn_type = connection.get_connection_type();
            self.components[component_id].current_representative(
                self.comp_to_cur_index_map[&component_id],
                conn_type,
                eq,
            );
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
        assert!(!self.locked, "Attempted to lock a locked circuit");
        self.locked = true;

        self.calc_current_index_map();
    }

    pub fn calc_current_index_map(&mut self) {
        assert!(self.locked);

        let len = self.components.iter().map(|c| c.num_eq()).sum::<usize>();
        let mut res = HashMap::with_capacity(len);

        let mut top_index = self.nodes.len(); // Since the first n indexes in M are the nodes and node currents
        for comp in self.components.iter() {
            res.insert(comp.get_id(), top_index);
            top_index += comp.num_eq();
        }

        self.comp_to_cur_index_map = res;
        self.num_variables = top_index;
    }
}

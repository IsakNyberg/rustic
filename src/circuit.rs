use crate::components::Component;
use crate::components::ComponentTrait;
use crate::components::Connection::{Connected, Disconnected};
use crate::components::ConnectionTrait;
use crate::components::ConnectionType;
use crate::components::ConnectionType::{Annode, Cathode, GroundConnection};
use crate::components::Identifer;
use crate::components::Node;

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

    pub fn from_components(name: String, id: usize, components: Vec<Component>) -> Self {
        Self {
            name,
            id,
            components,
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

    pub fn connect_nodes(&mut self, args: Vec<(usize, usize, ConnectionType)>) {
        for (comp_id, node_id, con_type) in args {
            self.connect_node(comp_id, node_id, con_type);
        }
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

    pub fn get_currents_at_node(&self, node_id: usize) -> Vec<(usize, f64)> {
        let mut terms = Vec::<(usize, f64)>::new();
        let node = &self.nodes[node_id];
        let num_nodes = self.nodes.len();
        for connection in node.connections.iter() {
            // everything whose input is the cathode is added to the current
            let component_id = (*connection).get_id();
            let con_type = connection.get_connection_type();
            match con_type {
                Annode => terms.push((num_nodes + component_id, 1.0)),
                Cathode => terms.push((num_nodes + component_id, -1.0)),
                GroundConnection => terms.push((num_nodes + component_id, -1.0)),
                conn_type => panic!("Unimplemented connection type {:?}", conn_type),
            }
        }
        terms
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

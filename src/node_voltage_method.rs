use crate::circuit;
use crate::components;
use crate::components::Component;
use crate::components::ComponentTrait;
use crate::components::Node;
use circuit::Circuit;
use components::Component::*;

use nalgebra::LU;
use nalgebra::{DMatrix, DVector};

/*
* this struct contains the nesseday information to solve a circuit using the node voltage method.
*/
#[derive(Debug, Clone)]
pub struct NodeVoltageMethod {
    pub circuit: Circuit,
    pub is_solved: bool,
    pub potentials: Vec<f64>,
    pub currents: Vec<f64>,
}

/*
* this impl block contains the methods to solve a circuit using the node voltage method.
*/
impl NodeVoltageMethod {
    /*
     * this method creates a new node voltage method struct.
     */
    pub fn new(circuit: Circuit) -> Self {
        Self {
            circuit,
            is_solved: false,
            potentials: Vec::new(),
            currents: Vec::new(),
        }
    }

    pub fn nodes(&self) -> &Vec<Node> {
        &self.circuit.nodes
    }

    pub fn components(&self) -> &Vec<Component> {
        &self.circuit.components
    }

    pub fn get_component(&self, id: usize) -> &Component {
        &self.circuit.components[id]
    }

    pub fn get_node(&self, id: usize) -> &Node {
        &self.circuit.nodes[id]
    }

    /*
     * this method solves the circuit using the node voltage method.
     */
    pub fn solve(&mut self) -> Result<(), String> {
        let num_unknowns = self.nodes().len() + self.components().len();
        let mut m: Vec<Vec<f64>> = Vec::with_capacity(num_unknowns);
        let mut b: Vec<f64> = Vec::new();
        let num_nodes = self.nodes().len();
        // fill matrix with equations for each node
        // M * x = b

        // do all current equations it is always that the sum of current of a component is 0 (KCL)
        for component in self.components() {
            println!("component: {:?}", component);
            let (terms1, terms2) = match component {
                ResistorComponent(resistor) => {
                    // V1-V2 = IR
                    let mut terms = Vec::<(usize, f64)>::new();
                    terms.push((resistor.node1, 1.0 / resistor.resistance));
                    terms.push((resistor.node2, -1.0 / resistor.resistance));
                    terms.push((num_nodes + resistor.get_id(), -1.0));
                    (terms, vec![])
                }
                DCVoltageSourceComponent(dc_vc) => {
                    // sum of all current into annode and cathode is 0
                    // this one case is special because it has a 2 equations
                    // first do the annode where current flows into the VS
                    let mut terms1 = Vec::<(usize, f64)>::new();
                    let annode = &self.get_node(dc_vc.annode);
                    for component_id in annode.connections.iter() {
                        // everything whose input is the annode is takes away from the current
                        if self
                            .get_component(*component_id)
                            .is_input_node(annode.get_id())
                        {
                            terms1.push((num_nodes + component_id, -1.0));
                        } else {
                            terms1.push((num_nodes + component_id, 1.0));
                        }
                        terms1.push((num_nodes + component_id, 1.0));
                    }
                    // then do the cathode where current flows out of the VS
                    let mut terms2 = Vec::<(usize, f64)>::new();
                    let cathode = &self.get_node(dc_vc.cathode);
                    for component_id in cathode.connections.iter() {
                        // everything whose input is the cathode is takes away from the current
                        if self
                            .get_component(*component_id)
                            .is_input_node(cathode.get_id())
                        {
                            terms2.push((num_nodes + component_id, -1.0));
                        } else {
                            terms2.push((num_nodes + component_id, 1.0));
                        }
                        terms2.push((num_nodes + component_id, 1.0));
                    }
                    (terms1, terms2)
                }
                GroundComponent(gnd) => {
                    // Current into ground should be 0
                    //let mut terms = Vec::<(usize, f64)>::new();
                    //terms.push((num_nodes + gnd.get_id(), 1.0));
                    (vec![], vec![])
                }
            };

            let mut equation1: Vec<f64> = vec![0.0; num_unknowns];
            let mut equation2: Vec<f64> = vec![0.0; num_unknowns];

            if terms1.len() > 0 {
                for (id, value) in terms1 {
                    equation1[id] = value;
                }
                m.push(equation1);
                b.push(0.0);
            }

            if terms2.len() > 0 {
                for (id, value) in terms2 {
                    equation2[id] = value;
                }
                m.push(equation2);
                b.push(0.0);
            }
        }

        // do all voltage equation voltage is always the difference between two nodes (KVL)
        for component in self.components() {
            let mut equation: Vec<f64> = vec![0.0; num_unknowns];
            println!("component: {:?}", component);
            let b_value;
            let terms = match component {
                ResistorComponent(_) => {
                    continue;
                }
                DCVoltageSourceComponent(dc_vc) => {
                    // V1-V2 = V
                    let mut terms = Vec::<(usize, f64)>::new();
                    terms.push((dc_vc.annode, 1.0));
                    terms.push((dc_vc.cathode, -1.0));
                    b_value = dc_vc.voltage;
                    terms
                }
                GroundComponent(gnd) => {
                    // V = 0
                    let mut terms = Vec::<(usize, f64)>::new();
                    terms.push((gnd.node, 1.0));
                    b_value = 0.0;
                    terms
                }
            };

            for (id, value) in terms {
                equation[id] = value;
            }
            m.push(equation);
            b.push(b_value);
        }

        // convert the matrix to an ndarray
        let rows = m.len();
        let cols = m[0].len();
        let mut a_vec = Vec::<f64>::with_capacity(rows * cols);
        for row in m.iter() {
            for col in row.iter() {
                a_vec.push(*col);
            }
        }
        let a = DMatrix::from_row_slice(rows, cols, &a_vec);
        let b = DVector::from_vec(b);

        // solve the matrix
        let lu = LU::new(a);
        let x = lu.solve(&b).expect("Failed to solve the linear system");
        println!("{:?}", x);

        // set the potentials of the nodes
        for mut node in self.circuit.nodes.iter_mut() {
            node.potential = x[node.get_id()];
            node.locked = true;
        }

        for i in 0..num_nodes {
            self.potentials.push(x[i]);
        }
        for i in num_nodes..num_unknowns {
            self.currents.push(x[i]);
        }
        self.is_solved = true;
        return Ok(());
    }
}

use crate::circuit;
use crate::components;
use crate::components::Component;
use crate::components::Node;
use circuit::Circuit;
use components::Component::*;

use nalgebra::LU;
//use nalgebra::SVD;
use nalgebra::{DMatrix, DVector};

/*
* this struct contains the nesseday information to solve a circuit using the node voltage method.
*/
#[derive(Debug, Clone)]
pub struct Solver {
    pub circuit: Circuit,
    pub is_solved: bool,
    pub potentials: Vec<f64>,
    pub currents: Vec<f64>,
}

/*
* this impl block contains the methods to solve a circuit using the node voltage method.
*/
impl Solver {
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
        for node_id in 0..self.circuit.nodes.len() {
            let mut equation: Vec<f64> = vec![0.0; num_unknowns];
            let node_terms = self.circuit.get_currents_at_node(node_id);
            for (id, value) in node_terms {
                equation[id] = value;
            }
            m.push(equation);
            b.push(0.0);
        }

        // do all voltage equation voltage is always the difference between two nodes (KVL)
        for component in self.components() {
            let mut equation: Vec<f64> = vec![0.0; num_unknowns];
            println!("component: {component:?}");
            let b_value;
            let terms = match component {
                ResistorComponent(resistor) => {
                    // V1-V2 = IR
                    let mut terms = Vec::<(usize, f64)>::new();
                    let recip_resistance = resistor.resistance.recip();
                    let term1 = self.circuit.get_potential_index(resistor.node1.get_id());
                    terms.push((term1, recip_resistance));
                    terms.push((resistor.node2.get_id(), -recip_resistance));
                    terms.push((num_nodes + resistor.get_id(), -1.0));
                    b_value = 0.0;
                    terms
                }
                DCVoltageSourceComponent(dc_vs) => {
                    // V1-V2 = V
                    let mut terms = Vec::<(usize, f64)>::new();
                    terms.push((dc_vs.anode.get_id(), 1.0));
                    terms.push((dc_vs.cathode.get_id(), -1.0));
                    b_value = dc_vs.voltage;
                    terms
                }
                GroundComponent(gnd) => {
                    // V = 0
                    let mut terms = Vec::<(usize, f64)>::new();
                    terms.push((gnd.node.get_id(), 1.0));
                    b_value = 0.0;
                    terms
                }
                DCCurrentSourceComponent(dc_cs) => {
                    // I = I
                    let mut terms = Vec::<(usize, f64)>::new();
                    terms.push((num_nodes + dc_cs.get_id(), 1.0));
                    b_value = dc_cs.current;
                    terms
                }
                SwitchSPDTComponent(switch) => {
                    // V_in = V_out
                    let mut terms = Vec::new();
                    let input = switch.get_input_id();
                    let output = switch.get_output_id();

                    terms.push((input, 1.0));
                    terms.push((output, -1.0));
                    b_value = 0.0;
                    terms
                }
                _ => panic!("Unimplemented component {self:?}"),
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
        println!("A: {a:?}");
        println!("b: {b:?}");

        // solve the matrix
        //let svd = SVD::new(a, true, true);
        let lu = LU::new(a);
        println!("LU: {lu:?}");

        // let x = svd.solve(&b, 1e-9).expect("Failed to solve");
        let x = lu.solve(&b).expect("Failed to solve the linear system");
        println!("x: {x:?}");

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

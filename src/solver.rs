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
        let num_unknowns = self.circuit.total_size;
        let mut m: Vec<Vec<f64>> = Vec::with_capacity(num_unknowns);
        let mut b: Vec<f64> = Vec::with_capacity(num_unknowns);
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
            for channel in 0..component.get_currents() {
                println!("component: {component:?} channel: {channel}");
                let mut equation: Vec<f64> = vec![0.0; num_unknowns];
                let b_value;
                match component {
                    ResistorComponent(resistor) => {
                        assert_eq!(channel, 0);
                        // V1-V2 = IR, rearranged:
                        // V1/R - V2/R - I = 0
                        let v1 = self.circuit.get_potential_index(resistor.node1.get_id());
                        let v2 = self.circuit.get_potential_index(resistor.node2.get_id());
                        let i = self.circuit.get_current_index(component);

                        let recip_resistance = resistor.resistance.recip();
                        equation[v1] = recip_resistance;
                        equation[v2] = -recip_resistance;
                        equation[i] = -1.0;
                        b_value = 0.0;
                    }
                    DCVoltageSourceComponent(dc_vs) => {
                        assert_eq!(channel, 0);
                        // V1-V2 = V
                        let v1 = self.circuit.get_potential_index(dc_vs.anode.get_id());
                        let v2 = self.circuit.get_potential_index(dc_vs.cathode.get_id());
                        equation[v1] = 1.0;
                        equation[v2] = -1.0;
                        b_value = dc_vs.voltage;
                    }
                    GroundComponent(gnd) => {
                        assert_eq!(channel, 0);
                        // V = 0
                        let v = self.circuit.get_potential_index(gnd.node.get_id());
                        equation[v] = 1.0;
                        b_value = 0.0;
                    }
                    DCCurrentSourceComponent(dc_cs) => {
                        assert_eq!(channel, 0);
                        // I = I
                        let i = self.circuit.get_potential_index(dc_cs.get_id());
                        equation[i] = 1.0;
                        b_value = dc_cs.current;
                    }
                    SwitchSPDTComponent(switch) => {
                        // V_in = V_out
                        // I_unused = 0
                        let input = switch.get_input_id();
                        let output = switch.get_output_id();

                        let vi = self.circuit.get_potential_index(input);
                        let vo = self.circuit.get_potential_index(output);

                        let i =
                            self.circuit.get_current_index(component) + switch.get_unused_offset();

                        match channel {
                            0 => {
                                // V_in = V_out
                                equation[vi] = 1.0;
                                equation[vo] = -1.0;
                                b_value = 0.0;
                            }
                            1 => {
                                // I_unused = 0
                                equation[i] = 1.0;
                                b_value = 0.0;
                            }
                            _ => unreachable!("Attempt adding more than two rows for SPDT switch"),
                        }
                    }
                    _ => panic!("Unimplemented component {self:?}"),
                }
                m.push(equation);
                b.push(b_value);
            }
        }

        // convert the matrix to an ndarray
        let rows = m.len();
        let cols = m[0].len();
        let mut a_vec = vec![0.0; rows*cols];
        a_vec.chunks_mut(cols).zip(m.iter())
            .inspect(|(_, source)| println!("{source:?}"))
            .for_each(|(dest, source)| dest.copy_from_slice(source));

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

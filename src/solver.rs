use crate::circuit;
use crate::components::Component;
use crate::components::Node;
use circuit::Circuit;

use nalgebra::LU;
// use nalgebra::QR;
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
        let num_unknowns = self.circuit.num_variables;
        let num_nodes = self.nodes().len();
        // fill matrix with equations for each node
        // M * x = b

        let mut m = vec![0.0; num_unknowns.pow(2)];
        let mut b = vec![0.0; num_unknowns];

        // KCL
        for (node_id, row) in m.chunks_mut(num_unknowns).enumerate().take(num_nodes) {
            self.circuit.currents_at_node_eq(node_id, row);
        }

        // Do the component-related equations
        // TODO make this readable
        self.components()
            .iter()
            .flat_map(|c| (0..c.get_currents()).map(move |eq_id| (c, eq_id)))
            .enumerate()
            .zip(m.chunks_mut(num_unknowns).skip(num_nodes))
            .for_each(|((row_id, (c, eq_id)), row)| {
                b[num_nodes + row_id] = c.equation(num_nodes + row_id - eq_id, row, eq_id);
            });

        for (r, bi) in m.chunks(num_unknowns).zip(&b) {
            for v in r {
                print!("{v:>6} ");
            }
            println!("   |    {bi}");
        }
        let a = DMatrix::from_row_slice(num_unknowns, num_unknowns, &m);
        let b = DVector::from_vec(b);

        // solve the matrix
        // let qr = QR::new(a);
        let lu = LU::new(a);

        // let x = qr.solve(&b).expect("Failed to solve the linear system");
        let x = lu.solve(&b).expect("Failed to solve the linear system");

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

#![allow(dead_code)]

mod circuit;
mod components;
mod node_voltage_method;

use circuit::Circuit;
use components::ConnectionType::{Annode, Cathode, GroundConnection};
use components::*;
use node_voltage_method::NodeVoltageMethod;

fn new_identifer(id: &mut usize) -> Identifer {
    let res = components::Identifer::from_id(*id);
    *id += 1;
    return res;
}

fn main() {
    // just build a simple circuit for now
    // this process will be improved at some potint
    let mut a: usize = 0;
    let component_id = &mut a;

    let components = vec![
        DCVoltageSourceComponent(DCVoltageSource::new(new_identifer(component_id), 5.0)),
        ResistorComponent(Resistor::new(new_identifer(component_id), 1000.0)),
        ResistorComponent(Resistor::new(new_identifer(component_id), 1000.0)),
        GroundComponent(Ground::new(new_identifer(component_id))),
    ];

    let mut circuit = Circuit::from_components("test".to_string(), 0, components);

    // (component_id, node_id, connection_type)
    let con_args = vec![
        (0, 0, Annode),
        (1, 0, Cathode),
        (1, 1, Annode),
        (2, 1, Cathode),
        (2, 2, Annode),
        (0, 2, Cathode),
        (3, 1, GroundConnection),
    ];
    circuit.connect_nodes(con_args);

    let mut nvm = NodeVoltageMethod::new(circuit);
    nvm.solve().expect("Failed to solve circuit");

    for i in 0..nvm.components().len() {
        println!(
            "Component: {}: {}A",
            nvm.get_component(i).get_name(),
            nvm.currents[i]
        );
    }
    for i in 0..nvm.nodes().len() {
        println!(
            "Node: {}: {}V",
            nvm.get_node(i).get_name(),
            nvm.potentials[i]
        );
    }
}

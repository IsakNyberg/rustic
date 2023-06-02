#![allow(dead_code)]

mod circuit;
mod components;
mod solver;

use circuit::Circuit;
use components::ConnectionType::{Anode, Cathode, GroundConnection, *};
use components::*;
use solver::Solver;

fn new_identifer(id: &mut usize) -> Identifer {
    let res = components::Identifer::from_id(*id);
    *id += 1;
    return res;
}

fn main() {
    // just build a simple circuit for now
    // this process will be improved at some point
    let mut a: usize = 0;
    let component_id = &mut a;

    let components: Vec<Component> = vec![
        Box::new(DCVoltageSource::new(new_identifer(component_id), 3.0)),
        Box::new(Resistor::new(new_identifer(component_id), 1000.0)),
        Box::new(Resistor::new(new_identifer(component_id), 2000.0)),
        Box::new(Ground::new(new_identifer(component_id))),
        Box::new(SwitchSPDT::new(new_identifer(component_id))),
        Box::new(DCVoltageSource::new(new_identifer(component_id), 3.0)),
        Box::new(Resistor::new(new_identifer(component_id), 10000.0)),
        Box::new(Resistor::new(new_identifer(component_id), 50000.0)),
        Box::new(SwitchSPDT::new(new_identifer(component_id))),
    ];

    let mut circuit = Circuit::from_components("test".to_string(), 0, components);

    // ((component_id, connection_type) (component_id, connection_type))
    // let connection_pairs = vec![
    //     ((0, Anode), (1, Cathode)),
    //     ((1, Anode), (2, Cathode)),
    //     ((0, Anode), (3, GroundConnection)),
    //     ((2, Anode), (0, Cathode)),
    // ];
    let connection_pairs = vec![
        ((0, Anode), (1, Cathode)),
        ((0, Anode), (2, Cathode)),
        ((1, Anode), (4, Left)),
        ((2, Anode), (4, Right)),
        ((4, Middle), (0, Cathode)),
        ((3, GroundConnection), (0, Anode)),
        ((3, GroundConnection), (5, Cathode)),
        ((0 + 5, Anode), (1 + 5, Cathode)),
        ((0 + 5, Anode), (2 + 5, Cathode)),
        ((1 + 5, Anode), (4 + 4, Left)),
        ((2 + 5, Anode), (4 + 4, Right)),
        ((4 + 4, Middle), (0 + 5, Cathode)),
    ];
    circuit.connect_components(connection_pairs);
    circuit.lock();

    let mut nvm = Solver::new(circuit);
    nvm.solve().expect("Failed to solve circuit");

    for (i, comp) in nvm.components().iter().enumerate() {
        for passage in 0..comp.num_eq() {
            println!(
                "Component: {}.{passage}: {:.6}A",
                comp.get_name(),
                nvm.currents[i + passage],
            );
        }
    }
    for i in 0..nvm.nodes().len() {
        println!(
            "Node: {}: {:.2}V",
            nvm.get_node(i).get_name(),
            nvm.potentials[i]
        );
    }
}

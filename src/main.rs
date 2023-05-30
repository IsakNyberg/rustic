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

    let mut components = vec![
        DCVoltageSourceComponent(DCVoltageSource::new(new_identifer(component_id), 5.0)),
        ResistorComponent(Resistor::new(new_identifer(component_id), 1000.0)),
        ResistorComponent(Resistor::new(new_identifer(component_id), 2000.0)),
        GroundComponent(Ground::new(new_identifer(component_id))),
        SwitchSPDTComponent(SwitchSPDT::new(new_identifer(component_id))),
    ];

    if let SwitchSPDTComponent(s) = &mut components[4] {
        s.toggle();  // optional switch toggle (it works!)
    }

    let mut circuit = Circuit::from_components("test".to_string(), 0, components);

    // ((component_id, connection_type) (component_id, connection_type))
    // let connection_pairs = vec![
    //     ((0, Anode), (1, Cathode)),
    //     ((1, Anode), (2, Cathode)),
    //     ((2, Cathode), (3, GroundConnection)),
    //     ((2, Anode), (0, Cathode)),
    // ];
    let connection_pairs = vec![
        ((0, Anode), (1, Cathode)),
        ((0, Anode), (2, Cathode)),
        ((1, Anode), (4, Output1)),
        ((2, Anode), (4, Output2)),
        ((4, Input1), (0, Cathode)),
        ((3, GroundConnection), (0, Anode)),
    ];
    circuit.connect_components(connection_pairs);
    circuit.lock();

    let mut nvm = Solver::new(circuit);
    nvm.solve().expect("Failed to solve circuit");

    for (i, comp) in nvm.components().iter().enumerate() {
        for passage in 0..comp.get_currents() {
            println!(
                "Component: {}.{passage}: {}A",
                comp.get_name(),
                nvm.currents[i + passage],
            );
        }
    }
    for i in 0..nvm.nodes().len() {
        println!(
            "Node: {}: {}V",
            nvm.get_node(i).get_name(),
            nvm.potentials[i]
        );
    }
}

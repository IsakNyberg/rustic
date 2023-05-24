#![allow(unused)]

mod circuit;
mod components;
mod node_voltage_method;

use circuit::Circuit;
use components::Component::*;
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
    let mut b: usize = 0;
    let node_id = &mut b;

    let mut nodes = vec![
        Node::new(new_identifer(node_id)),
        Node::new(new_identifer(node_id)),
    ];

    let mut components = vec![
        DCVoltageSourceComponent(DCVoltageSource::new(
            new_identifer(component_id),
            5.0,
            nodes[0].identifer.id,
            nodes[1].identifer.id,
        )),
        ResistorComponent(Resistor::new(
            new_identifer(component_id),
            1000.0,
            nodes[0].identifer.id,
            nodes[1].identifer.id,
        )),
        GroundComponent(Ground::new(
            new_identifer(component_id),
            nodes[1].identifer.id,
        )),
    ];

    nodes[0].connections.push(components[0].get_id());
    nodes[1].connections.push(components[0].get_id());
    nodes[0].connections.push(components[1].get_id());
    nodes[1].connections.push(components[1].get_id());
    nodes[1].connections.push(components[2].get_id());

    let mut circuit = Circuit::from_components_nodes("test".to_string(), 0, components, nodes);

    let mut nvm = NodeVoltageMethod::new(circuit);
    nvm.solve();

    for i in 0..nvm.components().len() {
        println!(
            "Component: {}: {}mA",
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

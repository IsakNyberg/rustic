use std::ops::Not;

use super::{
    ComponentTrait, Connection,
    Connection::*,
    ConnectionType::{self, *},
    Identifer,
};

#[derive(Debug, Clone)]
pub enum SwitchPosition {
    LeftPosition,
    RightPosition,
}

impl Not for SwitchPosition {
    type Output = SwitchPosition;

    fn not(self) -> Self::Output {
        use SwitchPosition::*;
        match self {
            LeftPosition => RightPosition,
            RightPosition => LeftPosition,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SwitchSPDT {
    pub identifer: Identifer,
    pub state: SwitchPosition,
    pub node_m: Connection,
    pub node_l: Connection,
    pub node_r: Connection,
}

// Single Pole Double Throw Switch
impl SwitchSPDT {
    pub fn new(identifer: Identifer) -> Self {
        Self {
            identifer,
            state: SwitchPosition::LeftPosition,
            node_l: Disconnected(Left),
            node_m: Disconnected(Middle),
            node_r: Disconnected(Right),
        }
    }
    pub fn toggle(&mut self) {
        self.state = !self.state.clone();
    }

    pub fn set(&mut self, state: SwitchPosition) {
        self.state = state;
    }

    pub fn get_output_id(&self) -> usize {
        match self.state {
            SwitchPosition::LeftPosition => self.node_l.get_id(),
            SwitchPosition::RightPosition => self.node_r.get_id(),
        }
    }

    pub fn get_unused_offset(&self) -> usize {
        // which current channel is the unused one with current switch
        match self.state {
            SwitchPosition::LeftPosition => 1,
            SwitchPosition::RightPosition => 0,
        }
    }

    pub fn get_input_id(&self) -> usize {
        self.node_m.get_id()
    }
}

const PANIC_TEXT: &'static str = "SPDT Switch can only has connection type Left Right and Middle";

impl ComponentTrait for SwitchSPDT {
    fn get_id(&self) -> usize {
        self.identifer.id
    }

    fn get_name(&self) -> String {
        self.identifer.name.clone()
    }

    fn connect(&mut self, node: usize, connection_type: ConnectionType) {
        match connection_type {
            Left => self.node_l = Connected(node, Left),
            Middle => self.node_m = Connected(node, Middle),
            Right => self.node_r = Connected(node, Right),
            _ => unimplemented!("{PANIC_TEXT}"),
        }
    }

    fn disconnect(&mut self, connection_type: ConnectionType) {
        match connection_type {
            Left => self.node_l = Disconnected(Left),
            Middle => self.node_m = Disconnected(Middle),
            Right => self.node_r = Disconnected(Right),
            _ => unimplemented!("{PANIC_TEXT}"),
        }
    }

    fn get_connection(&self, connection_type: ConnectionType) -> Connection {
        match connection_type {
            Left => self.node_l.clone(),
            Middle => self.node_m.clone(),
            Right => self.node_r.clone(),
            _ => unimplemented!("{PANIC_TEXT}"),
        }
    }

    fn current_representative(&self, index: usize, conn_type: ConnectionType, eq: &mut [f64]) {
        // This is complex, depending on which connection that asks and the state of the switch
        // we return different currents to the node
        // current 0 is the current that goes middle to left
        // current 1 is the current that goes middle to right
        match conn_type {
            Left => {
                eq[index] = 1.0; // current flows out of the left "output"
            }
            Middle => {
                eq[index] = -1.0; // current flows into of the left "output"
                eq[index + 1] = -1.0; // current flows into of the right "output"
            }
            Right => {
                eq[index + 1] = 1.0; // current flows out of the right "output"
            }
            _ => unimplemented!("{PANIC_TEXT}"),
        }
    }

    fn num_eq(&self) -> usize {
        2
    }

    fn equation(&self, offset: usize, equation: &mut [f64], eq_id: usize) -> f64 {
        // This is complex, we have two equations:
        // 1. the potential of the middle node is the same as the selected node
        // 2. the current into the unselected node is 0
        assert!(eq_id < self.num_eq());
        if eq_id == 0 {
            // Potential on each side of the switch selected is the same
            // v0 - v1 = 0
            let v0 = self.get_input_id();
            let v1 = self.get_output_id();
            equation[v0] = 1.0;
            equation[v1] = -1.0;
            0.0
        } else {
            // Current into the unselected node is 0
            // i0 = 0
            let i0 = self.get_unused_offset();
            equation[offset + i0] = 1.0;
            0.0
        }
    }
}

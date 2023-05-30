use std::ops::Not;

use super::{
    Connection,
    Connection::*,
    ConnectionType::{self, *},
    Identifer,
};

#[derive(Debug, Clone)]
pub enum SwitchPosition {
    Left,
    Right,
}

impl Not for SwitchPosition {
    type Output = SwitchPosition;

    fn not(self) -> Self::Output {
        use SwitchPosition::*;
        match self {
            Left => Right,
            Right => Left,
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
            state: SwitchPosition::Left,
            node_l: Connection::Disconnected(Output1),
            node_m: Connection::Disconnected(Input1),
            node_r: Connection::Disconnected(Output2),
        }
    }

    pub fn connect(&mut self, con: &Connection) {
        match con {
            Connected(_, Output1) | Disconnected(Output1) => self.node_l = *con,
            Connected(_, Input1) | Disconnected(Input1) => self.node_m = *con,
            Connected(_, Output2) | Disconnected(Output2) => self.node_r = *con,
            _ => panic!("Invalid Switch connection"),
        }
    }

    pub fn get_connection(&self, connection_type: ConnectionType) -> Connection {
        match connection_type {
            Output1 => self.node_l.clone(),
            Input1 => self.node_m.clone(),
            Output2 => self.node_r.clone(),
            _ => panic!("Invalid Switch connection"),
        }
    }

    pub fn get_id(&self) -> usize {
        self.identifer.id
    }

    pub fn toggle(&mut self) {
        self.state = !self.state.clone();
    }

    pub fn set(&mut self, state: SwitchPosition) {
        self.state = state;
    }

    pub fn get_output_id(&self) -> usize {
        match self.state {
            SwitchPosition::Left => self.node_l.get_id(),
            SwitchPosition::Right => self.node_r.get_id(),
        }
    }

    pub fn get_unused_offset(&self) -> usize {
        // which current channel is the unused one with current switch
        match self.state {
            SwitchPosition::Left => 1,
            SwitchPosition::Right => 0,
        }
    }

    pub fn get_input_id(&self) -> usize {
        self.node_m.get_id()
    }
}

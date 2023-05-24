use super::Identifer;

/*
* This struct represents a DC voltage source in a circuit.
* It has an id, voltage, max_current, and annode and a cathode.
*/

#[derive(Debug, Clone)]
pub struct DCVoltageSource {
    pub identifer: Identifer,
    pub voltage: f64,
    pub annode: usize,
    pub cathode: usize,
}

impl DCVoltageSource {
    pub fn new(
        identifer: Identifer,
        voltage: f64,
        annode: usize,
        cathode: usize,
    ) -> DCVoltageSource {
        DCVoltageSource {
            identifer,
            voltage,
            annode,
            cathode,
        }
    }

    pub fn set_annode(&mut self, annode: usize) {
        self.annode = annode;
    }

    pub fn set_cathode(&mut self, cathode: usize) {
        self.cathode = cathode;
    }

    pub fn get_id(&self) -> usize {
        self.identifer.id
    }
}

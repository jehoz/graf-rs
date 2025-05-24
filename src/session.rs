use std::collections::HashMap;

use macroquad::math::Vec2;

use crate::{
    dag::{Dag, VertexId},
    devices::{Clock, Device, Gate, Note},
};

pub struct Session {
    pub devices: HashMap<VertexId, Box<dyn Device>>,
    pub circuit: Dag,
}

impl Session {
    pub fn new() -> Self {
        Session {
            devices: HashMap::new(),
            circuit: Dag::new(),
        }
    }

    pub fn create_clock(&mut self, position: Vec2) {
        let clock = Clock::new(position);
        let vid = self.circuit.add_vertex();
        self.devices.insert(vid, Box::new(clock));
    }

    pub fn create_gate(&mut self, position: Vec2) {
        let gate = Gate::new(position);
        let vid = self.circuit.add_vertex();
        self.devices.insert(vid, Box::new(gate));
    }

    pub fn create_note(&mut self, position: Vec2) {
        let note = Note::new(position);
        let vid = self.circuit.add_vertex();
        self.devices.insert(vid, Box::new(note));
    }
}

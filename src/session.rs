use std::collections::HashMap;

use macroquad::math::Vec2;

use crate::{
    dag::{Dag, VertexId},
    devices::{ClockDevice, Device, GateDevice, NoteDevice},
};

pub struct Node {
    pub position: Vec2,
    pub device: Device,
}

pub struct Session {
    pub devices: HashMap<VertexId, Node>,
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
        let clock = Node {
            position,
            device: Device::Clock(ClockDevice::default()),
        };
        let vid = self.circuit.add_vertex();
        self.devices.insert(vid, clock);
    }

    pub fn create_gate(&mut self, position: Vec2) {
        let gate = Node {
            position,
            device: Device::Gate(GateDevice::default()),
        };
        let vid = self.circuit.add_vertex();
        self.devices.insert(vid, gate);
    }

    pub fn create_note(&mut self, position: Vec2) {
        let note = Node {
            position,
            device: Device::Note(NoteDevice::default()),
        };
        let vid = self.circuit.add_vertex();
        self.devices.insert(vid, note);
    }
}

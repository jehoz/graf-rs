use macroquad::math::Vec2;

use crate::{
    dag::Dag,
    devices::{ClockDevice, Device, GateDevice, NoteDevice},
};

pub struct Node {
    pub position: Vec2,
    pub device: Device,
}

pub struct Session {
    nodes: Dag<Node>,
}

impl Session {
    pub fn new() -> Self {
        Session { nodes: Dag::new() }
    }

    pub fn create_clock(&mut self, position: Vec2) {
        let clock = Node {
            position,
            device: Device::Clock(ClockDevice::default()),
        };
        self.nodes.add_vertex(clock);
    }

    pub fn create_gate(&mut self, position: Vec2) {
        let gate = Node {
            position,
            device: Device::Gate(GateDevice::default()),
        };
        self.nodes.add_vertex(gate);
    }

    pub fn create_note(&mut self, position: Vec2) {
        let note = Node {
            position,
            device: Device::Note(NoteDevice::default()),
        };
        self.nodes.add_vertex(note);
    }
}

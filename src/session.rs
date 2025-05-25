use std::collections::HashMap;

use macroquad::{color::WHITE, math::Vec2, shapes::draw_line};

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

    pub fn add_device(&mut self, device: Box<dyn Device>) {
        let id = self.circuit.add_vertex();
        self.devices.insert(id, device);
    }

    pub fn connect_devices(&mut self, from: VertexId, to: VertexId) {
        // just silently ignore any errors for now
        let _ = self.circuit.add_edge((from, to));
    }

    pub fn get_device_at(&self, point: Vec2) -> Option<VertexId> {
        for (id, device) in self.devices.iter() {
            if device.is_point_inside(point) {
                return Some(*id);
            }
        }
        return None;
    }

    pub fn device_position(&self, id: VertexId) -> Option<Vec2> {
        self.devices.get(&id).map(|d| d.get_position())
    }

    pub fn move_device(&mut self, device_id: VertexId, position: Vec2) {
        if let Some(device) = self.devices.get_mut(&device_id) {
            device.set_position(position);
        }
    }

    pub fn draw(&self) {
        for (from, to) in self.circuit.edges() {
            let from_pos = self.device_position(*from).unwrap();
            let to_pos = self.device_position(*to).unwrap();
            draw_line(from_pos.x, from_pos.y, to_pos.x, to_pos.y, 1.0, WHITE);
        }

        for device in self.devices.values() {
            device.draw();
        }
    }
}

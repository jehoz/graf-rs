use std::collections::HashMap;

use macroquad::{color::WHITE, math::Vec2};

use crate::{
    dag::{Dag, Edge, VertexId},
    devices::{Arity, Device},
    drawing_utils::draw_wire_between_devices,
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

    pub fn disconnect_devices(&mut self, from: VertexId, to: VertexId) {
        self.circuit.remove_edge((from, to))
    }

    pub fn get_device_at(&self, point: Vec2) -> Option<VertexId> {
        for (id, device) in self.devices.iter() {
            if device.is_point_inside(point) {
                return Some(*id);
            }
        }
        None
    }

    pub fn get_wire_at(&self, point: Vec2) -> Option<Edge> {
        const WIRE_CLICKABLE_DISTANCE: f32 = 5.0;

        for (from_id, to_id) in self.circuit.edges() {
            let u = self.device_position(*from_id).unwrap();
            let v = self.device_position(*to_id).unwrap();

            let len2 = u.distance_squared(v);

            if len2 == 0.0 {
                return None;
            }

            let t = ((point - u).dot(v - u) / len2).clamp(0.0, 1.0);
            let point_on_line = u + t * (v - u);

            if point.distance(point_on_line) < WIRE_CLICKABLE_DISTANCE {
                return Some((*from_id, *to_id));
            }
        }

        None
    }

    pub fn can_connect(&self, from: VertexId, to: VertexId) -> bool {
        let to_dev = self.devices.get(&to).unwrap();
        if to_dev.input_arity() == Arity::Nullary {
            return false;
        }

        !self.circuit.is_reachable(to, from)
    }

    pub fn device_position(&self, id: VertexId) -> Option<Vec2> {
        self.devices.get(&id).map(|d| d.get_position())
    }

    pub fn move_device(&mut self, device_id: VertexId, position: Vec2) {
        if let Some(device) = self.devices.get_mut(&device_id) {
            device.set_position(position);
        }
    }

    pub fn update(&mut self) {
        let mut device_outputs: HashMap<VertexId, bool> = HashMap::new();
        for dev_id in self.circuit.vertices() {
            let inputs: Vec<bool> = self
                .circuit
                .parents(*dev_id)
                .filter_map(|from_id| device_outputs.get(from_id).copied())
                .collect();

            let dev = self.devices.get_mut(dev_id).unwrap();
            if let Some(output) = dev.update(inputs) {
                device_outputs.insert(*dev_id, output);
            }
        }
    }

    pub fn draw(&self) {
        for device in self.devices.values() {
            device.draw();
        }

        for (from_id, to_id) in self.circuit.edges() {
            let from_dev = self.devices.get(&from_id).unwrap();
            let to_dev = self.devices.get(&to_id).unwrap();
            draw_wire_between_devices(from_dev.as_ref(), to_dev.as_ref(), WHITE);
        }
    }
}

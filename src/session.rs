use std::{collections::HashMap, time::Instant};

use macroquad::{
    color::{Color, BLACK, RED, WHITE},
    math::{Rect, Vec2},
};

use crate::{
    dag::{Dag, DeviceId, Edge},
    devices::{Arity, Device},
    drawing_utils::draw_wire_between_devices,
    midi::MidiConfig,
};

pub struct UpdateContext {
    pub t0: Instant,

    pub bpm: u32,

    pub midi_config: MidiConfig,
}

impl UpdateContext {
    pub fn new() -> Self {
        UpdateContext {
            t0: Instant::now(),
            bpm: 120,

            midi_config: MidiConfig::new(),
        }
    }
}

pub struct DrawContext {
    pub fg_color: Color,
    pub bg_color: Color,
    pub err_color: Color,
}

impl DrawContext {
    pub fn new() -> Self {
        DrawContext {
            fg_color: WHITE,
            bg_color: BLACK,
            err_color: RED,
        }
    }
}

pub struct Session {
    pub devices: HashMap<DeviceId, Box<dyn Device>>,
    pub selected: Vec<DeviceId>,

    pub circuit: Dag,

    pub update_ctx: UpdateContext,
    pub draw_ctx: DrawContext,
}

impl Session {
    pub fn new() -> Self {
        Session {
            devices: HashMap::new(),
            selected: Vec::new(),
            circuit: Dag::new(),

            update_ctx: UpdateContext::new(),
            draw_ctx: DrawContext::new(),
        }
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) {
        let id = self.circuit.add_vertex();
        self.devices.insert(id, device);
    }

    pub fn connect_devices(&mut self, from: DeviceId, to: DeviceId) {
        // just silently ignore any errors for now
        let _ = self.circuit.add_edge((from, to));
    }

    pub fn disconnect_devices(&mut self, from: DeviceId, to: DeviceId) {
        self.circuit.remove_edge((from, to))
    }

    pub fn get_device_at(&self, point: Vec2) -> Option<DeviceId> {
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

    pub fn can_connect(&self, from: DeviceId, to: DeviceId) -> bool {
        let to_dev = self.devices.get(&to).unwrap();
        if to_dev.input_arity() == Arity::Nullary {
            return false;
        }

        !self.circuit.is_reachable(to, from)
    }

    pub fn device_position(&self, id: DeviceId) -> Option<Vec2> {
        self.devices.get(&id).map(|d| d.get_position())
    }

    pub fn move_device(&mut self, device_id: DeviceId, position: Vec2) {
        if let Some(device) = self.devices.get_mut(&device_id) {
            device.set_position(position);
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected.clear();
    }

    pub fn select_device(&mut self, device_id: DeviceId) {
        if !self.selected.contains(&device_id) {
            self.selected.push(device_id);
        }
    }

    pub fn select_devices_in_rect(&mut self, rect: Rect) {
        for (id, device) in self.devices.iter() {
            if rect.contains(device.get_position()) {
                self.selected.push(*id);
            }
        }
    }

    pub fn delete_selected_devices(&mut self) {
        for dev_id in &self.selected {
            self.circuit.remove_vertex(*dev_id);
            self.devices.remove(&dev_id);
        }

        self.clear_selection();
    }

    pub fn update(&mut self) {
        let mut device_outputs: HashMap<DeviceId, bool> = HashMap::new();
        for dev_id in self.circuit.vertices() {
            let inputs: Vec<bool> = self
                .circuit
                .parents(*dev_id)
                .filter_map(|from_id| device_outputs.get(from_id).copied())
                .collect();

            let dev = self.devices.get_mut(dev_id).unwrap();
            if let Some(output) = dev.update(&mut self.update_ctx, inputs) {
                device_outputs.insert(*dev_id, output);
            }
        }
    }

    pub fn draw(&self) {
        for (dev_id, device) in &self.devices {
            device.draw(&self.draw_ctx, self.selected.contains(dev_id));
        }

        for (from_id, to_id) in self.circuit.edges() {
            let from_dev = self.devices.get(&from_id).unwrap();
            let to_dev = self.devices.get(&to_id).unwrap();
            draw_wire_between_devices(from_dev.as_ref(), to_dev.as_ref(), self.draw_ctx.fg_color);
        }
    }
}

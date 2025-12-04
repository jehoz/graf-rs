use core::clone::Clone;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use macroquad::math::{Rect, Vec2};

use crate::{
    app::DrawContext,
    dag::{self, Dag, DeviceId, Wire, WireType},
    devices::{Arity, Device},
    drawing_utils::draw_wire_between_devices,
};

const SNAP_GRID_SIZE: f32 = 16.0;

pub struct UpdateContext {
    pub beat_clock: f32,
    pub free_clock: Duration,
    pub bpm: u32,

    pub this_update: Instant,
    pub last_update: Instant,

    pub is_paused: bool,
}

impl UpdateContext {
    pub fn new() -> Self {
        UpdateContext {
            beat_clock: 0.0,
            free_clock: Duration::ZERO,
            bpm: 120,

            this_update: Instant::now(),
            last_update: Instant::now(),

            is_paused: false,
        }
    }
}

pub struct Session {
    pub devices: HashMap<DeviceId, Box<dyn Device>>,
    pub circuit: Dag,

    pub selected: Vec<DeviceId>,
    pub clipboard: (HashMap<DeviceId, Box<dyn Device>>, Vec<Wire>),

    pub update_ctx: UpdateContext,
}

impl Session {
    pub fn new() -> Self {
        Session {
            devices: HashMap::new(),
            circuit: Dag::new(),

            selected: Vec::new(),
            clipboard: (HashMap::new(), Vec::new()),

            update_ctx: UpdateContext::new(),
        }
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) -> DeviceId {
        let id = self.circuit.add_device();
        self.devices.insert(id, device);
        self.snap_device_to_grid(id);
        id
    }

    pub fn connect_devices(&mut self, from: DeviceId, to: DeviceId, wire_type: WireType) {
        // just silently ignore any errors for now
        if let Err(dag::IllegalWireError) = self.circuit.add_wire(from, to, wire_type) {
            print!("Got IllegalEdgeError when trying to connected devices!!!");
        }
    }

    pub fn disconnect_devices(&mut self, from: DeviceId, to: DeviceId) {
        self.circuit.remove_wire(from, to)
    }

    pub fn get_device_at(&self, position: Vec2) -> Option<DeviceId> {
        for (id, device) in self.devices.iter() {
            if device.is_point_inside(position) {
                return Some(*id);
            }
        }
        None
    }

    pub fn get_wire_at(&self, position: Vec2) -> Option<Wire> {
        const WIRE_CLICKABLE_DISTANCE: f32 = 5.0;

        for edge in self.circuit.wires() {
            let u = self.device_position(edge.from).unwrap();
            let v = self.device_position(edge.to).unwrap();

            let len2 = u.distance_squared(v);

            if len2 == 0.0 {
                return None;
            }

            let t = ((position - u).dot(v - u) / len2).clamp(0.0, 1.0);
            let point_on_line = u + t * (v - u);

            if position.distance(point_on_line) < WIRE_CLICKABLE_DISTANCE {
                return Some(*edge);
            }
        }

        None
    }

    pub fn can_connect(&self, from: DeviceId, to: DeviceId) -> bool {
        let to_dev = self.devices.get(&to).unwrap();
        if to_dev.input_arity() == Arity::Nullary {
            return false;
        } else if to_dev.input_arity() == Arity::Unary && self.circuit.incoming(to).count() > 0 {
            return false;
        }

        !self.circuit.is_reachable(to, from)
    }

    pub fn device_position(&self, id: DeviceId) -> Option<Vec2> {
        self.devices.get(&id).map(|d| d.get_position())
    }

    pub fn move_device(&mut self, device_id: DeviceId, delta: Vec2) {
        if let Some(device) = self.devices.get_mut(&device_id) {
            let new_pos = device.get_position() + delta;
            device.set_position(new_pos);
        }
    }

    pub fn move_device_to(&mut self, device_id: DeviceId, position: Vec2) {
        if let Some(device) = self.devices.get_mut(&device_id) {
            device.set_position(position);
        }
    }

    pub fn snap_device_to_grid(&mut self, device_id: DeviceId) {
        if let Some(device) = self.devices.get_mut(&device_id) {
            let snapped = (device.get_position() / SNAP_GRID_SIZE).round() * SNAP_GRID_SIZE;
            device.set_position(snapped);
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

    pub fn move_selected_devices(&mut self, delta: Vec2) {
        for dev_id in self.selected.iter() {
            let pos = self.device_position(*dev_id).unwrap() + delta;
            self.devices.get_mut(dev_id).map(|d| d.set_position(pos));
        }
    }

    pub fn snap_selected_to_grid(&mut self) {
        let selected = self.selected.clone();
        for dev_id in selected {
            self.snap_device_to_grid(dev_id);
        }
    }

    pub fn delete_selected_devices(&mut self) {
        for dev_id in &self.selected {
            self.circuit.remove_device(*dev_id);
            self.devices.remove(&dev_id);
        }

        self.clear_selection();
    }

    pub fn copy_selected_devices(&mut self) {
        let mut devices = HashMap::new();
        let mut top_left = Vec2::new(f32::INFINITY, f32::INFINITY);
        for dev_id in &self.selected {
            if let Some(device) = self.devices.get(&dev_id) {
                let pos = device.get_position();
                if pos.x < top_left.x {
                    top_left.x = pos.x;
                }
                if pos.y < top_left.y {
                    top_left.y = pos.y;
                }
                devices.insert(*dev_id, device.clone_dyn());
            }
        }

        // set device positions to be relative to bounding box top-left corner
        for (_, device) in devices.iter_mut() {
            device.set_position(device.get_position() - top_left);
        }

        let mut edges = Vec::new();
        for edge in self.circuit.wires() {
            if devices.contains_key(&edge.from) && devices.contains_key(&edge.to) {
                edges.push(*edge);
            }
        }

        self.clipboard = (devices, edges);
    }

    pub fn paste_clipboard(&mut self, position: Vec2) {
        let (devices, edges) = &self.clipboard;

        let mut new_devices = HashMap::new();
        for (id, device) in devices.iter() {
            new_devices.insert(*id, device.clone_dyn());
        }
        let edges = edges.clone();

        let mut dev_id_map = HashMap::new();
        for (old_id, device) in new_devices.drain() {
            let new_id = self.add_device(device);
            self.move_device(new_id, position);
            dev_id_map.insert(old_id, new_id);
        }

        for edge in edges.clone().iter() {
            let from = dev_id_map.get(&edge.from).unwrap();
            let to = dev_id_map.get(&edge.to).unwrap();
            self.connect_devices(*from, *to, edge.wire_type);
        }

        self.clear_selection();
        for dev_id in dev_id_map.values() {
            self.select_device(*dev_id);
        }
    }

    pub fn toggle_pause(&mut self) {
        self.update_ctx.is_paused = !self.update_ctx.is_paused;
    }

    pub fn reset(&mut self) {
        self.update_ctx.beat_clock = 0.0;
        self.update_ctx.free_clock = Duration::ZERO;
        self.update_ctx.last_update = Instant::now();

        for dev in self.devices.values_mut() {
            dev.reset();
        }
    }

    pub fn update(&mut self) {
        self.update_ctx.this_update = Instant::now();

        if !self.update_ctx.is_paused {
            let time_elapsed = self.update_ctx.this_update - self.update_ctx.last_update;
            let beats_elapsed = time_elapsed.as_secs_f32() * (self.update_ctx.bpm as f32 / 60.0);

            self.update_ctx.free_clock += time_elapsed;
            self.update_ctx.beat_clock += beats_elapsed;
        }

        let mut device_outputs: HashMap<DeviceId, bool> = HashMap::new();
        for dev_id in self.circuit.devices() {
            let inputs: Vec<bool> = self
                .circuit
                .incoming(*dev_id)
                .filter_map(|wire| match wire.wire_type {
                    WireType::Normal => device_outputs.get(&wire.from).copied(),
                    WireType::Negated => device_outputs.get(&wire.from).copied().map(|x| !x),
                })
                .collect();

            let dev = self.devices.get_mut(dev_id).unwrap();
            if let Some(output) = dev.update(&mut self.update_ctx, inputs) {
                device_outputs.insert(*dev_id, output);
            }
        }

        self.update_ctx.last_update = self.update_ctx.this_update;
    }

    pub fn draw(&self, draw_ctx: &DrawContext) {
        for wire in self.circuit.wires() {
            let from_dev = self.devices.get(&wire.from).unwrap();
            let to_dev = self.devices.get(&wire.to).unwrap();
            draw_wire_between_devices(
                draw_ctx,
                from_dev.as_ref(),
                to_dev.as_ref(),
                wire.wire_type,
                draw_ctx.colors.fg_1,
            );
        }

        for (dev_id, device) in &self.devices {
            // temporary until i move position out of device struct
            let pos = device.get_position();

            device.draw(
                draw_ctx,
                draw_ctx.world_to_viewport(pos),
                24.0,
                self.selected.contains(dev_id),
            );
        }
    }
}

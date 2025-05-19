use std::collections::HashMap;

use macroquad::math::Vec2;

use crate::devices::Device;

#[derive(Eq, Hash, PartialEq)]
pub struct NodeId(u32);

pub struct Node {
    pub position: Vec2,
    pub device: Device,
}

pub struct Session {
    node_id_counter: u32,
    pub nodes: HashMap<NodeId, Node>,
}

impl Session {
    pub fn new() -> Self {
        Session {
            node_id_counter: 0,
            nodes: HashMap::new(),
        }
    }
}

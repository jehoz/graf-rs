use macroquad::prelude::Vec2;

pub mod clock;
pub mod gate;
pub mod note;

const CLOCK_RADIUS: f32 = 18.0;
const GATE_WIDTH: f32 = 36.0;
const NOTE_RADIUS: f32 = 18.0;

#[derive(PartialEq)]
pub enum Arity {
    Nullary,
    Unary,
    NAry,
}

pub trait Device {
    fn get_position(&self) -> Vec2;
    fn set_position(&mut self, pos: Vec2);
    fn closest_border_point(&self, point: Vec2) -> Vec2;

    fn is_point_inside(&self, pt: Vec2) -> bool;

    fn draw(&self);

    // number of input wires that can be plugged into the device
    fn input_arity(&self) -> Arity;

    // can there be wires coming out of this device?
    fn has_output(&self) -> bool;
}

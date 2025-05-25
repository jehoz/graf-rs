use macroquad::{
    color::WHITE,
    math::Vec2,
    shapes::{draw_circle, draw_circle_lines, draw_rectangle_lines},
};

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

    fn is_point_inside(&self, pt: Vec2) -> bool;

    fn draw(&self);

    // number of input wires that can be plugged into the device
    fn input_arity(&self) -> Arity;

    // can there be wires coming out of this device?
    fn has_output(&self) -> bool;
}

pub struct Clock {
    position: Vec2,

    frequency: Frequency,
    duty_cycle: f32,
    offset: f32,
}

impl Clock {
    pub fn new(position: Vec2) -> Self {
        Clock {
            position,
            frequency: Frequency::Beats(BeatFraction {
                numerator: 1,
                denominator: 4,
            }),
            duty_cycle: 0.5,
            offset: 0.,
        }
    }
}

impl Device for Clock {
    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    fn is_point_inside(&self, pt: Vec2) -> bool {
        self.position.distance(pt) <= CLOCK_RADIUS
    }

    fn draw(&self) {
        let Vec2 { x, y } = self.position;
        draw_circle_lines(x, y, CLOCK_RADIUS, 1.0, WHITE);
    }

    fn input_arity(&self) -> Arity {
        Arity::Nullary
    }

    fn has_output(&self) -> bool {
        true
    }
}

pub struct Gate {
    position: Vec2,
    operation: BooleanOperation,
}

impl Gate {
    pub fn new(position: Vec2) -> Self {
        Gate {
            position,
            operation: BooleanOperation::AND,
        }
    }
}

impl Device for Gate {
    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    fn is_point_inside(&self, pt: Vec2) -> bool {
        let dx = (pt.x - self.position.x).abs();
        let dy = (pt.y - self.position.y).abs();
        dx <= GATE_WIDTH && dy <= GATE_WIDTH
    }

    fn draw(&self) {
        let Vec2 { x, y } = self.position;
        draw_rectangle_lines(
            x - GATE_WIDTH / 2.,
            y - GATE_WIDTH / 2.,
            GATE_WIDTH,
            GATE_WIDTH,
            2.0,
            WHITE,
        );
    }

    fn input_arity(&self) -> Arity {
        Arity::NAry
    }

    fn has_output(&self) -> bool {
        true
    }
}

pub struct Note {
    position: Vec2,

    midi_note: u8,
    velocity: u8,
    is_on: bool,
}

impl Note {
    pub fn new(position: Vec2) -> Self {
        Note {
            position,
            midi_note: 60,
            velocity: 100,
            is_on: false,
        }
    }
}

impl Device for Note {
    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    fn is_point_inside(&self, pt: Vec2) -> bool {
        self.position.distance(pt) <= NOTE_RADIUS
    }

    fn draw(&self) {
        let Vec2 { x, y } = self.position;
        draw_circle(x, y, NOTE_RADIUS, WHITE);
    }

    fn input_arity(&self) -> Arity {
        Arity::Unary
    }

    fn has_output(&self) -> bool {
        false
    }
}

pub enum BooleanOperation {
    AND,
    OR,
    XOR,
    NAND,
    NOR,
    XNOR,
}

pub enum Frequency {
    Milliseconds(f32),
    Beats(BeatFraction),
}

pub struct BeatFraction {
    numerator: u16,
    denominator: u16,
}

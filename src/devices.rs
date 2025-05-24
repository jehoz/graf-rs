use macroquad::math::Vec2;

pub trait Device {
    fn get_position(&self) -> Vec2;
    fn set_position(&mut self, pos: Vec2);
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

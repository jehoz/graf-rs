pub enum Device {
    Clock(ClockDevice),
    Gate(GateDevice),
    Note(NoteDevice),
}

pub struct ClockDevice {
    frequency: Frequency,
    duty_cycle: f32,
    offset: f32,
}

impl Default for ClockDevice {
    fn default() -> Self {
        ClockDevice {
            frequency: Frequency::Beats(BeatFraction {
                numerator: 1,
                denominator: 4,
            }),
            duty_cycle: 0.5,
            offset: 0.,
        }
    }
}

pub struct GateDevice {
    operation: BooleanOperation,
}

impl Default for GateDevice {
    fn default() -> Self {
        GateDevice {
            operation: BooleanOperation::AND,
        }
    }
}

pub struct NoteDevice {
    midi_note: u8,
    velocity: u8,
    is_on: bool,
}

impl Default for NoteDevice {
    fn default() -> Self {
        NoteDevice {
            midi_note: 60,
            velocity: 100,
            is_on: false,
        }
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

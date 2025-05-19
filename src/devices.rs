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

pub struct GateDevice {
    operation: BooleanOperation,
}

pub struct NoteDevice {
    midi_note: u8,
    velocity: u8,
    is_on: bool,
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

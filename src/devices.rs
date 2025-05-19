enum Device {
    Clock(ClockDevice),
    Gate(GateDevice),
    Note(NoteDevice),
}

struct ClockDevice {
    frequency: Frequency,
    duty_cycle: f32,
    offset: f32,
}

struct GateDevice {
    operation: BooleanOperation,
}

struct NoteDevice {
    midi_note: u8,
    velocity: u8,
    is_on: bool,
}

enum BooleanOperation {
    AND,
    OR,
    XOR,
    NAND,
    NOR,
    XNOR,
}

enum Frequency {
    Milliseconds(f32),
    Beats(BeatFraction),
}

struct BeatFraction {
    numerator: u16,
    denominator: u16,
}

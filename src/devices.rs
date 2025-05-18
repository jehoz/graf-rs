enum Device {
    Clock(ClockDevice),
    Gate(GateDevice),
    Note(NoteDevice),
}

struct ClockDevice {
    frequency: f32,
    duty_cycle: f32,
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

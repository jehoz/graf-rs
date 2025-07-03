use std::io::{stdin, stdout, Write};

use midir::{MidiOutput, MidiOutputConnection, MidiOutputPorts};
use midly::live::LiveEvent;

pub struct MidiConfig {
    // pub midi_out: MidiOutput,
    // pub ports: MidiOutputPorts,
    pub connection: Option<MidiOutputConnection>,
}

impl MidiConfig {
    pub fn new() -> Self {
        let midi_out = MidiOutput::new("graf").unwrap();

        let ports = midi_out.ports();

        let out_port = match ports.len() {
            0 => panic!("No output MIDI ports found!!!"),
            1 => {
                println!("Connecting to: {}", midi_out.port_name(&ports[0]).unwrap());
                &ports[0]
            }
            _ => {
                println!("Available ports:");
                for (i, p) in ports.iter().enumerate() {
                    println!("{}: {}", i, midi_out.port_name(p).unwrap());
                }
                print!("Please select output port: ");
                stdout().flush().unwrap();
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                ports
                    .get(input.trim().parse::<usize>().unwrap())
                    .ok_or("invalid output port selected")
                    .unwrap()
            }
        };

        let connection = midi_out.connect(out_port, "graf-midi").unwrap();

        MidiConfig {
            // midi_out,
            // ports,
            connection: Some(connection),
        }
    }

    pub fn handle_live_event(&mut self, event: LiveEvent) {
        match &mut self.connection {
            Some(conn) => {
                let mut buf = Vec::new();
                event.write(&mut buf).unwrap();
                conn.send(&buf).unwrap();
            }

            None => println!("Tried to send MIDI message but no connection"),
        }
    }
}

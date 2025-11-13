use core::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

use midir::{MidiOutput, MidiOutputConnection, MidiOutputPorts};
use midly::live::LiveEvent;
use midly::num::u4;
use midly::MidiMessage;

// using this type alias because LiveEvents need lifetimes and I don't
// want that to pollute my other types
pub type MidiEvent = (u4, MidiMessage);

#[derive(Clone)]
pub struct MidiEventSender {
    event_queue: Rc<RefCell<VecDeque<MidiEvent>>>
}

impl MidiEventSender {
    pub fn send(&self, event: MidiEvent) {
        self.event_queue.borrow_mut().push_back(event);
    }
}

pub struct MidiConfig {
    // pub midi_out: MidiOutput,
    // pub ports: MidiOutputPorts,
    pub connection: Option<MidiOutputConnection>,

    event_queue: Rc<RefCell<VecDeque<MidiEvent>>>,
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

            event_queue: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    pub fn process_events(&mut self) {
        match &mut self.connection {
            Some(conn) => {
                for (channel, message) in self.event_queue.borrow_mut().drain(..) {
                    let mut buf = Vec::new();
                    let event = LiveEvent::Midi { channel, message };
                    event.write(&mut buf).unwrap();
                    conn.send(&buf).unwrap();
                }
            }

            None => println!("Tried to send MIDI message but no connection"),
        }
    }

    pub fn get_event_sender(&self) -> MidiEventSender {
        MidiEventSender {
            event_queue: self.event_queue.clone()
        }
    }
}

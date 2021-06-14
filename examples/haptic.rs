//! This is the example from the lib.rs documentation.

use pasts::prelude::*;
use stick::{Controller, Event};

async fn event_loop() {
    let mut listener = Controller::listener();
    let mut ctlrs = Vec::<Controller>::new();
    let mut left_rumble: f32 = 0.0;
    let mut right_rumble: f32 = 0.0;

    'e: loop {
        match poll![listener, poll!(ctlrs)].await.1 {
            (_, Event::Connect(new)) => {
                println!(
                    "Connected p{}, id: {:04X}_{:04X}_{:04X}_{:04X}, name: {}",
                    ctlrs.len() + 1,
                    new.id()[0],
                    new.id()[1],
                    new.id()[2],
                    new.id()[3],
                    new.name(),
                );
                ctlrs.push(*new);
            }
            (id, Event::Disconnect) => {
                println!("Disconnected p{}", id + 1);
                ctlrs.swap_remove(id);
            }
            (id, Event::Home(true)) => {
                println!("p{} ended the session", id + 1);
                break 'e;
            }
            (id, event) => {
                println!("p{}: {}", id + 1, event);
                match event {
                    Event::ActionA(pressed) => {
                        ctlrs[id].rumble(if pressed { 1.0 } else { 0.0 });
                    }
                    Event::ActionB(pressed) => {
                        ctlrs[id].rumble(if pressed { 0.3 } else { 0.0 });
                    }
                    Event::BumperL(pressed) => {
                        if pressed {
                            left_rumble = 1.0;
                        } else {
                            left_rumble = 0.0;
                        }

                        ctlrs[id].rumbles(left_rumble, right_rumble);
                    }
                    Event::BumperR(pressed) => {
                        if pressed {
                            right_rumble = 1.0;
                        } else {
                            right_rumble = 0.0;
                        }
                        ctlrs[id].rumbles(left_rumble, right_rumble);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    exec!(event_loop());
}

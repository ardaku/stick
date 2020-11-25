//! This is the example from the lib.rs documentation.

use pasts::prelude::*;
use stick::{Event, Controller};

async fn event_loop() {
    let mut listener = Controller::listener();
    let mut controllers = Vec::<Controller>::new();
    'e: loop {
        match poll![listener, poll!(controllers)].await.1 {
            (_, Event::Connect(new)) => {
                println!(
                    "Connected p{}, id: {:04X}_{:04X}_{:04X}_{:04X}, name: {}",
                    controllers.len() + 1,
                    new.id()[0],
                    new.id()[1],
                    new.id()[2],
                    new.id()[3],
                    new.name(),
                );
                controllers.push(*new);
            }
            (id, Event::Disconnect) => {
                println!("Disconnected p{}", id + 1);
                controllers.swap_remove(id);
            }
            (id, Event::Home(true)) => {
                println!("p{} ended the session", id + 1);
                break 'e;
            }
            (id, event) => {
                println!("p{}: {}", id + 1, event);
                match event {
                    Event::ActionA(pressed) => {
                        controllers[id].rumble(if pressed { 1.0 } else { 0.0 });
                    }
                    Event::ActionB(pressed) => {
                        controllers[id].rumble(if pressed {
                            0.25
                        } else {
                            0.0
                        });
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

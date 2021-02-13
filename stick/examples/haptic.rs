//! This is the example from the lib.rs documentation.

use stick::{Controller, Event};
use pasts::{race, wait};

async fn event_loop() {
    let mut listener = Controller::listener();
    let mut ctlrs = Vec::<Controller>::new();
    'e: loop {
        let event = wait![(&mut listener).await, race!(ctlrs)];
        match event {
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
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    pasts::block_on(event_loop());
}

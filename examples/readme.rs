// Example from the README.

use pasts::{prelude::*, CvarExec};
use stick::{Event, Hub, Pad};

async fn event_loop() {
    let mut hub = Hub::new();
    let mut pads = Vec::<Pad>::new();
    'e: loop {
        match [hub.fut(), pads.select().fut()].select().await.1 {
            (_, Event::Connect(pad)) => {
                println!(
                    "Connected p{}, id: {:X}, name: {}",
                    pads.len() + 1,
                    pad.id(),
                    pad.name(),
                );
                pads.push(*pad);
            }
            (id, Event::Disconnect) => {
                println!("Disconnected p{}", id + 1);
                pads.swap_remove(id);
            }
            (id, Event::Cmd) => {
                println!("p{} ended the session", id + 1);
                break 'e;
            }
            (id, event) => {
                println!("p{}: {}", id + 1, event);
                match event {
                    Event::ActA(pressed) => {
                        pads[id].rumble(if pressed { 1.0 } else { 0.0 });
                    }
                    Event::ActB(pressed) => {
                        pads[id].rumble(if pressed { 0.25 } else { 0.0 });
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    static EXECUTOR: CvarExec = CvarExec::new();

    EXECUTOR.block_on(event_loop())
}

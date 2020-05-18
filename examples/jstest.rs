//! Example is in the public domain.

use pasts::{CvarExec, prelude::*};
use stick::{Event, Gamepad, Port};

async fn event_loop() {
    let mut port = Port::new();
    let mut gamepads = Vec::<Gamepad>::new();
    'e: loop {
        match [port.fut(), gamepads.select().fut()].select().await.1 {
            (_, Event::Connect(gamepad)) => {
                println!(
                    "Connected p{}, id: {:X}, name: {}",
                    gamepads.len() + 1,
                    gamepad.id(),
                    gamepad.name(),
                );
                gamepads.push(*gamepad);
            }
            (id, Event::Disconnect) => {
                println!("Disconnected p{}", id + 1);
                gamepads.swap_remove(id);
            }
            (id, Event::Quit) => {
                println!("p{} ended the session", id + 1);
                break 'e;
            }
            (id, event) => {
                println!("p{}: {}", id + 1, event);
                match event {
                    Event::Accept(pressed) => {
                        gamepads[id].rumble(if pressed { 0.5 } else { 0.0 });
                    }
                    Event::Cancel(pressed) => {
                        gamepads[id].rumble(if pressed { 1.0 } else { 0.0 });
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

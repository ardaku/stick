// Example from the README.

use pasts::{prelude::*, CvarExec};
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
            (id, Event::Cmd) => {
                println!("p{} ended the session", id + 1);
                break 'e;
            }
            (id, event) => {
                println!("p{}: {}", id + 1, event);
                match event {
                    Event::Primary(pressed) => {
                        gamepads[id].rumble(if pressed { 1.0 } else { 0.0 });
                    }
                    Event::Secondary(pressed) => {
                        gamepads[id].rumble(if pressed { 0.25 } else { 0.0 });
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

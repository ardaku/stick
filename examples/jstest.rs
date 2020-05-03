use pasts::prelude::*;
use stick::{Event, Gamepad, Port};

async fn event_loop() {
    let mut port = Port::new();
    let mut gamepads = Vec::<Gamepad>::new();
    'e: loop {
        match [(&mut port).fut(), gamepads.select().fut()]
            .select()
            .await
            .1
        {
            (_, Event::Connect(gamepad)) => {
                println!(
                    "Connected p{}, id: {:X}, name: {}",
                    gamepads.len(),
                    gamepad.id(),
                    gamepad.name(),
                );
                gamepads.push(*gamepad);
            }
            (id, Event::Disconnect) => {
                println!("Disconnected p{}", id);
                gamepads.swap_remove(id);
            }
            (id, Event::Quit) => {
                println!("p{} ended the session", id);
                break 'e;
            }
            (id, event) => {
                println!("p{}: {}", id, event);
                match event {
                    Event::Accept(pressed) => {
                        gamepads[id].rumble(if pressed { 0.25 } else { 0.0 });
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
    pasts::ThreadInterrupt::block_on(event_loop())
}

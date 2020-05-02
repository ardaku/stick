use pasts::prelude::*;
use stick::{Event, Port, Gamepad};

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
                    "Connected p{}, id: {:X}",
                    gamepads.len(),
                    gamepad.id()
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
            }
        }
    }
}

fn main() {
    pasts::ThreadInterrupt::block_on(event_loop())
}

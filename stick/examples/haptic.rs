//! This is the example from the lib.rs documentation.

use pasts::Loop;
use std::task::Poll::{self, Pending, Ready};
use stick::{Controller, Event, Listener};

type Exit = usize;

struct State {
    listener: Listener,
    controllers: Vec<Controller>,
    rumble: (f32, f32),
}

impl State {
    fn connect(&mut self, controller: Controller) -> Poll<Exit> {
        println!(
            "Connected p{}, id: {:016X}, name: {}",
            self.controllers.len() + 1,
            controller.id(),
            controller.name(),
        );
        self.controllers.push(controller);
        Pending
    }

    fn event(&mut self, id: usize, event: Event) -> Poll<Exit> {
        let player = id + 1;
        println!("p{}: {}", player, event);
        match event {
            Event::Disconnect => {
                self.controllers.swap_remove(id);
            }
            Event::MenuR(true) => return Ready(player),
            Event::ActionA(pressed) => {
                self.controllers[id].rumble(f32::from(u8::from(pressed)));
            }
            Event::ActionB(pressed) => {
                self.controllers[id].rumble(0.5 * f32::from(u8::from(pressed)));
            }
            Event::BumperL(pressed) => {
                self.rumble.0 = f32::from(u8::from(pressed));
                self.controllers[id].rumble(self.rumble);
            }
            Event::BumperR(pressed) => {
                self.rumble.1 = f32::from(u8::from(pressed));
                self.controllers[id].rumble(self.rumble);
            }
            _ => {}
        }
        Pending
    }
}

async fn event_loop() {
    let mut state = State {
        listener: Listener::default(),
        controllers: Vec::new(),
        rumble: (0.0, 0.0),
    };

    let player_id = Loop::new(&mut state)
        .when(|s| &mut s.listener, State::connect)
        .poll(|s| &mut s.controllers, State::event)
        .await;

    println!("p{} ended the session", player_id);
}

fn main() {
    pasts::block_on(event_loop());
}

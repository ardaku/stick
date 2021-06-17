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
            "Connected p{}, id: {:04X}_{:04X}_{:04X}_{:04X}, name: {}",
            self.controllers.len() + 1,
            controller.id()[0],
            controller.id()[1],
            controller.id()[2],
            controller.id()[3],
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
            Event::Next(true) => return Ready(player),
            Event::ActionA(pressed) => {
                self.controllers[id].rumble(if pressed { 1.0 } else { 0.0 });
            }
            Event::ActionB(pressed) => {
                self.controllers[id].rumble(if pressed { 1.0 } else { 0.0 });
            }
            Event::BumperL(pressed) => {
                self.rumble.0 = if pressed { 1.0 } else { 0.0 };
                self.controllers[id].rumble(self.rumble);
            }
            Event::BumperR(pressed) => {
                self.rumble.1 = if pressed { 1.0 } else { 0.0 };
                self.controllers[id].rumble(self.rumble);
            }
            _ => {}
        }
        Pending
    }
}

async fn event_loop() {
    let mut state = State {
        listener: Listener::new(),
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

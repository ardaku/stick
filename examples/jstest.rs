use pasts;
use stick::Gamepads;

struct AppState {
    running: bool,
    gamepads: Gamepads,
    ctlrs: Vec<Pin<Box<&dyn StdGamepad>>>,
}

async fn connect(state: &mut AppState) {
    // Wait for a new gamepad to be plugged in.
    let ctlr = state.connections.await;
    // Add gamepad to list of controllers.
    state.ctlrs.push(Box::pin(ctlr));
}

async fn ctlr_event(state: &mut AppState) {
    // Poll all of the plugged in controllers at once.
    pasts::tasks!(while true; &state.ctlrs);
    //

    let id = state.port.input().await.unwrap(); // FIXME
    if let Some(state) = state.port.get(id) {
        println!("{}: {}", id, state);
    }
}

async fn async_main() {
    let mut state = AppState {
        running: true,
        gamepads: Gamepads::new(),
        ctlrs: Vec::new(),
    };
    // Look for new connections while checking current gamepads.
    pasts::tasks!(state while state.running; [connect, ctlr_event]);
}

fn main() {
    <pasts::ThreadInterrupt as pasts::Interrupt>::block_on(async_main())
}

use stick::Port;
use pasts;

struct AppState {
    running: bool,
    port: Port,
}

async fn ctlr_event(state: &mut AppState) {
    let id = state.port.input().await.unwrap(); // FIXME
    if let Some(state) = state.port.get(id) {
        println!("{}: {}", id, state);
    }
}

async fn async_main() {
    // Connect to all devices.
    let mut state = AppState {
        running: true,
        port: Port::new(),
    };
    pasts::run!(state while state.running; ctlr_event);
}

fn main() {
    <pasts::ThreadInterrupt as pasts::Interrupt>::block_on(async_main())
}

use stick::Port;

fn main() {
    // Connect to all devices.
    let mut port = Port::new();

    // Loop showing state of all devices.
    loop {
        // Cycle through all currently plugged in devices.
        let id = if let Some(a) = port.poll() {
            a
        } else {
            continue;
        };

        if let Some(state) = port.get(id) {
            println!("{}: {}", id, state);
        }
    }
}

use stick::Port;

fn main() {
    // Connect to all devices.
    let mut port = Port::new();

    port.update();

    // Loop showing state of all devices.
    loop {
        println!("Looking...");

        // Cycle through all currently plugged in devices.
        let id = port.poll();

        println!("{}: {}", id, port.get(id));

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

use stick::Port;

fn main() {
    // Connect to all devices.
    let mut port = Port::new();

//    port.update();

    // Loop showing state of all devices.
    loop {
        // Cycle through all currently plugged in devices.
        let id = if let Some(a) = port.poll() { a } else { continue };

        println!("{}: {}", id, port.get(id));

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

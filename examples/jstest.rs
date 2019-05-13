use stick::Port;

fn main() {
    // Connect to all devices.
    let mut port = Port::new();

    // Loop showing state of all devices.
    loop {
        // Cycle through all currently plugged in devices.
        for i in 0..port.update() {
            println!("{}: {}", i, port.get(i));
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

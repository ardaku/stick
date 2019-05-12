use stick::Port;

/*fn remapper(input: (usize, stick::Input)) -> (usize, stick::Input) {
    (input.0, match input.1 {
        stick::Input::ThrottleL(y) => { stick::Input::Camera(0.0, y) },
        a => a
    })
}*/

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

    /*	let mut cm = stick::ControllerManager::new(vec![
    /*		stick::Remapper::new(0x_07b5_0316, remapper)*/
        ]);

        loop {
            while let Some((j, i)) = cm.update() {
                println!("{}: {}", j, i);
            }
        }*/
}

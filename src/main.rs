use evdev::{uinput::VirtualDeviceBuilder, EventType, InputEvent, Key};
use std::time::Duration;

fn main() {
    let flags = xflags::parse_or_exit! {
        /// The device name
        optional -d, --device device: String
        /// List all available devices
        optional -l, --list
    };

    if flags.list {
        let mut devices = evdev::enumerate().map(|t| t.1).collect::<Vec<_>>();
        devices.reverse();
        devices.iter().enumerate().for_each(|(i, d)| {
            println!("{i}: {}", d.name().unwrap_or("Unnamed"));
        });
        std::process::exit(0);
    }

    let Some(device) = flags.device else {
        eprintln!("Error: no device specified");
        std::process::exit(1);
    };

    let Some((_path, mut d)) = evdev::enumerate()
        .into_iter()
        .find(|d| d.1.name().is_some_and(|n| n == device))
    else {
        eprintln!("Error: specified device doesn't exist, or invalid permissions");
        std::process::exit(1);
    };

    let mut device = VirtualDeviceBuilder::new()
        .unwrap()
        .name("remapped mouse")
        .with_keys(&d.supported_keys().unwrap())
        .unwrap()
        .build()
        .unwrap();

    loop {
        for ev in d.fetch_events().unwrap() {
            if ev.code() == 282 && ev.value() == 1 {
                device
                    .emit(&[InputEvent::new(EventType::KEY, Key::BTN_LEFT.0, 1)])
                    .unwrap();
                device
                    .emit(&[InputEvent::new(EventType::KEY, Key::BTN_LEFT.0, 0)])
                    .unwrap();

                std::thread::sleep(Duration::from_millis(50));

                device
                    .emit(&[InputEvent::new(EventType::KEY, Key::BTN_LEFT.0, 1)])
                    .unwrap();
                device
                    .emit(&[InputEvent::new(EventType::KEY, Key::BTN_LEFT.0, 0)])
                    .unwrap();
                // device.emit(&[]).unwrap();
            }
        }
    }
}

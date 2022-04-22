use desktop_man::utilities::*;
use desktop_man::Desktop;

fn main() {
    let desktop = Desktop::new();
    desktop.grab_key(41, 0);
    loop {
        let event = desktop.get_event();
        match event {
            Event::KeyPress(key) => {
                println!("Key Press{}", key.keycode);
            }
            Event::KeyRelease(key) => {
                println!("Key Release{}", key.keycode);
            }
            _ => {}
        }
    }
}

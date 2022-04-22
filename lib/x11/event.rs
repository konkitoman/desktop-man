use x11_man::xlib;

use crate::event::*;

use super::X11Session;

impl TEvent for X11Session {
    fn get_event(&self) -> Event {
        let event = self.display.next_event();
        match event {
            xlib::Event::KeyPress(key) => Event::KeyPress(Key {
                x: key.x,
                y: key.y,
                keycode: Keycode(key.keycode.0),
            }),
            xlib::Event::KeyRelease(key) => Event::KeyRelease(Key {
                x: key.x,
                y: key.y,
                keycode: Keycode(key.keycode.0),
            }),

            _ => Event::Unknown,
        }
    }
}

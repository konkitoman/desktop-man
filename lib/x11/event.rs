use x11_man::xlib::{self, TInput};

use crate::event::*;

use super::X11Session;

pub trait FromXEvent {
    fn from_xevent(event: &xlib::Event) -> Self;
}

impl FromXEvent for Event {
    fn from_xevent(event: &xlib::Event) -> Self {
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

impl TEvent for X11Session {
    fn get_event(&self) -> Event {
        let event = self.display.next_event();
        Event::from_xevent(&event)
    }

    fn try_get_event(&self) -> Option<Event> {
        let event = self.display.try_next_event();
        if let Some(event) = event {
            Some(Event::from_xevent(&event))
        } else {
            None
        }
    }
}

use crate::{
    event::{Event, TEvent},
    input::TInput,
    session::Session,
};

pub struct Desktop {
    pub session: Session,
}

impl Desktop {
    pub fn new() -> Self {
        let session = Session::new();
        Self { session }
    }
}

impl TInput for Desktop {
    fn grab_key(&self, keycode: u32, modifiers: u32) {
        match self.session {
            Session::X11(ref x11) => x11.grab_key(keycode, modifiers),
        }
    }

    fn grab_button(&self, button: u32, modifiers: u32, mask: u32) {
        match self.session {
            Session::X11(ref x11) => x11.grab_button(button, modifiers, mask),
        }
    }

    fn ungrab_key(&self, keycode: u32, modifiers: u32) {
        match self.session {
            Session::X11(ref x11) => x11.ungrab_key(keycode, modifiers),
        }
    }

    fn ungrab_button(&self, button: u32, modifiers: u32) {
        match self.session {
            Session::X11(ref x11) => x11.ungrab_button(button, modifiers),
        }
    }
}

impl TEvent for Desktop {
    fn get_event(&self) -> Event {
        match self.session {
            Session::X11(ref x11) => x11.get_event(),
        }
    }
}

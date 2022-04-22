use x11_man::{ffi::x, xlib::XDisplay};

mod event;
mod input;

#[derive(Debug, Copy, Clone)]
pub struct X11Session {
    pub display: XDisplay,
    pub root: x::Window,
}

impl X11Session {
    pub fn new() -> X11Session {
        let display = XDisplay::new(None);
        let root = display.root_window(display.default_screen());
        X11Session { display, root }
    }
}

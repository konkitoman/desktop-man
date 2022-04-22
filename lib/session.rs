use crate::x11::X11Session;

#[derive(Debug, Copy, Clone)]
pub enum Session {
    X11(X11Session),
}

impl Session {
    pub fn new() -> Session {
        Session::X11(X11Session::new())
    }
}

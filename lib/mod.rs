mod desktop;
pub mod event;
pub mod input;
pub mod session;

pub mod x11;

pub use x11_man;

pub use desktop::Desktop;

pub mod utilities {
    use super::*;
    pub use event::*;
    pub use input::*;
}

use std::fmt::{Display, Formatter};

pub trait TEvent {
    fn get_event(&self) -> Event;
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    KeyPress(Key),
    KeyRelease(Key),
    ButtonPress(Button),
    ButtonRelease(Button),
    Motion(Motion),
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct Key {
    pub x: i32,
    pub y: i32,
    pub keycode: Keycode,
}
#[derive(Debug, Clone, Copy)]
pub struct Button {}
#[derive(Debug, Clone, Copy)]
pub struct Focus {}
#[derive(Debug, Clone, Copy)]
pub struct Motion {}

#[derive(Debug, Copy, Clone)]
pub struct Keycode(pub u32);
impl Display for Keycode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Keycode: {}", self.0)
    }
}
impl PartialEq for Keycode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialEq<u32> for Keycode {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}
impl PartialEq<i32> for Keycode {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other as u32
    }
}

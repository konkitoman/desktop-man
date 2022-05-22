use x11_man::{
    x::{ButtonGrabConfig, KeyGrabConfig},
    xlib::TInput,
};

use super::X11Session;

impl crate::input::TInput for X11Session {
    fn grab_key(&self, keycode: u32, modifiers: u32) {
        let mut config = KeyGrabConfig::new();
        config.keycode = keycode as i32;
        config.modifiers = modifiers;
        self.display.grab_key(&config);
    }
    fn grab_button(&self, button: u32, modifiers: u32) {
        let mut config = ButtonGrabConfig::new();
        config.button = button;
        config.modifiers = modifiers;
        self.display.grab_button(&config);
    }

    fn ungrab_key(&self, keycode: u32, modifiers: u32) {
        self.display.ungrap_key(self.root, keycode, modifiers);
    }

    fn ungrab_button(&self, button: u32, modifiers: u32) {
        self.display.ungrab_button(self.root, button, modifiers);
    }
}

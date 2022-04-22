use super::X11Session;
use crate::input::TInput;

impl TInput for X11Session {
    fn grab_key(&self, keycode: u32, modifiers: u32) {
        self.display.grab_key(keycode as i32, modifiers, self.root);
    }
    fn grab_button(&self, button: u32, modifiers: u32, mask: u32) {
        self.display
            .grab_button(button, modifiers, self.root, mask, 0, 0);
    }

    fn ungrab_key(&self, keycode: u32, modifiers: u32) {
        self.display.ungrap_key(self.root, keycode, modifiers);
    }

    fn ungrab_button(&self, button: u32, modifiers: u32) {
        self.display.ungrab_button(self.root, button, modifiers);
    }
}

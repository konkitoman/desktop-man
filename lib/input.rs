pub trait TInput {
    fn grab_key(&self, keycode: u32, modifiers: u32);
    fn grab_button(&self, button: u32, modifiers: u32);
    fn ungrab_key(&self, keycode: u32, modifiers: u32);
    fn ungrab_button(&self, button: u32, modifiers: u32);
}

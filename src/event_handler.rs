use winit::platform::unix::x11::ffi::XK_F5;

pub trait EventHandler {
    fn on_scroll(&mut self, value: f64);
    fn on_char_input(&mut self, value: char);
}

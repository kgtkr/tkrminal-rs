#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGBColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub trait Renderer {
    fn draw_text(&mut self, text: &str, size: f32, x: f32, y: f32, color: RGBColor);
    fn width(&mut self) -> f32;
    fn height(&mut self) -> f32;
    fn clear_background(&mut self, color: RGBColor);
    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: RGBColor);
    fn measure_text(&mut self, text: &str, size: f32, max_width: f32) -> Vec<Vec<(f32, f32)>>;
    fn draw_sub_screen(
        &mut self,
        origin_x: f32,
        origin_y: f32,
        w: f32,
        h: f32,
        to_x: f32,
        to_y: f32,
    );
}

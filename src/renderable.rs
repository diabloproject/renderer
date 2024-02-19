pub type Color = [u8; 4];

#[derive(Copy, Clone, Debug)]
pub struct PointInfo {
    pub x:  f64,
    pub y:  f64,
    pub t:  f64,
    pub dx: f64,
    pub dy: f64,
    pub dt: f64,
}
pub trait Renderable {
    fn render(&self, point_info: PointInfo) -> Color;
}

impl<T> Renderable for T where T: Fn(PointInfo) -> Color {
    fn render(&self, point_info: PointInfo) -> Color {
        self(point_info)
    }
}
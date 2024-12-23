use std::time::Duration;

#[derive(Debug, Copy, Clone, Hash)]
pub struct Color {
    value: [u8; 4],
}

impl Default for Color {
    fn default() -> Self {
        [0, 0, 0, 255].into()
    }
}

impl From<[u8; 4]> for Color {
    fn from(value: [u8; 4]) -> Self {
        Self { value }
    }
}

impl From<Color> for [u8; 4] {
    fn from(value: Color) -> Self {
        value.value
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PointInfo {
    pub x: f64,
    pub y: f64,
    pub t: Duration,
}
pub trait Renderable {
    fn render(&self, point_info: PointInfo) -> Color;
}

impl<T: Fn(PointInfo) -> Color> Renderable for T {
    fn render(&self, point_info: PointInfo) -> Color {
        self(point_info)
    }
}
